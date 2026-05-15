use clap::Parser;
use firewall_core::domain::packet::Protocol;
use firewall_core::rules::geo_rules::GeoBlockRule;
use firewall_core::rules::ip_rules::IpBlocklistRule;
use firewall_core::rules::port_rules::{PortBlocklistRule, Service, WellKnownServicesRule};
use firewall_core::rules::rate_limit_rules::rate_limit_rules::RateLimitRule;
use firewall_core::{
    Action, Firewall, FirewallBuilder, MqttPublisher, MqttPublisherConfig, Packet,
};
use firewall_core::domain::rate_limiter::{RateLimitConfig, RateLimitKeyType};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, warn};

#[derive(Parser, Debug, Clone)]
#[command(name = "firewall-daemon")]
#[command(about = "Turing anomaly-detection edge firewall daemon", long_about = None)]
struct Cli {
    #[arg(long, default_value_t = false)]
    no_mqtt: bool,

    #[arg(long, default_value = "127.0.0.1")]
    mqtt_host: String,

    #[arg(long, default_value_t = 1883)]
    mqtt_port: u16,

    #[arg(long, default_value = "turing/router/flows")]
    flow_topic: String,

    #[arg(long, default_value = "turing/router/stats")]
    stats_topic: String,

    #[arg(long, default_value_t = 5)]
    publish_interval_secs: u64,

    #[arg(long, default_value_t = 50)]
    synthetic_packets: usize,
}

fn install_baseline_rules(fw: &Firewall) {
    let admin_block = PortBlocklistRule::new("block-admin-ports")
        .add_ports([23u16, 21, 3389])
        .with_priority(100);
    fw.add_rule(Box::new(admin_block));

    let iot_allowlist = WellKnownServicesRule::new("iot-services")
        .allow_iot_services()
        .with_priority(90);
    fw.add_rule(Box::new(iot_allowlist));

    let bogon = IpBlocklistRule::new("bogon-sources")
        .add_ip(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)))
        .add_ip(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
        .with_priority(95);
    fw.add_rule(Box::new(bogon));

    let geo = GeoBlockRule::new("blocked-ranges")
        .add_range(
            IpAddr::V4(Ipv4Addr::new(203, 0, 113, 0)),
            IpAddr::V4(Ipv4Addr::new(203, 0, 113, 255)),
            "TEST-NET-3",
        )
        .with_priority(80);
    fw.add_rule(Box::new(geo));

    let rate_limit = RateLimitRule::new(
        "per-src-rate-limit",
        RateLimitConfig {
            rate: 50.0,
            capacity: 100.0,
            key_type: RateLimitKeyType::SourceIp,
        },
    )
    .with_priority(70);
    fw.add_rule(Box::new(rate_limit));
}

fn synthetic_packets(n: usize) -> Vec<Packet> {
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let src = IpAddr::V4(Ipv4Addr::new(10, 0, 0, (i % 250) as u8 + 1));
        let dst = IpAddr::V4(Ipv4Addr::new(10, 0, 1, 1));
        let (proto, dport) = match i % 5 {
            0 => (Protocol::Tcp, Service::Ssh.port()),
            1 => (Protocol::Tcp, Service::Telnet.port()),
            2 => (Protocol::Tcp, Service::Https.port()),
            3 => (Protocol::Udp, Service::Dns.port()),
            _ => (Protocol::Tcp, Service::Mqtt.port()),
        };
        out.push(
            Packet::new(src)
                .with_destination(dst)
                .with_ports(40000 + (i as u16), dport)
                .with_protocol(proto)
                .with_payload(vec![0u8; 64]),
        );
    }
    out
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli = Cli::parse();

    info!("Starting firewall daemon (v{})", firewall_core::VERSION);
    let fw = Arc::new(FirewallBuilder::new(Action::Block).build());
    install_baseline_rules(&fw);
    info!(rules = fw.list_rules().len(), "Baseline rules installed");

    if !cli.no_mqtt {
        let config = MqttPublisherConfig {
            broker_host: cli.mqtt_host.clone(),
            broker_port: cli.mqtt_port,
            client_id: "firewall-router".to_string(),
            flow_topic: cli.flow_topic.clone(),
            stats_topic: cli.stats_topic.clone(),
            interval: Duration::from_secs(cli.publish_interval_secs),
            keep_alive: Duration::from_secs(15),
        };
        let (publisher, eventloop) = MqttPublisher::new(config);
        let _handles = publisher.spawn(eventloop, fw.flow_tracker(), fw.stats_collector());
        info!(
            host = %cli.mqtt_host,
            port = cli.mqtt_port,
            "MQTT publisher started"
        );
    } else {
        info!("MQTT disabled (--no-mqtt)");
    }

    let packets = synthetic_packets(cli.synthetic_packets);
    let total = packets.len();
    let mut allowed = 0u64;
    let mut blocked = 0u64;
    for pkt in &packets {
        match fw.process_packet(pkt) {
            Action::Allow | Action::Log => allowed += 1,
            Action::Block => blocked += 1,
        }
    }
    info!(total, allowed, blocked, "TSA-precheck evaluation complete");

    let stats = fw.get_stats();
    info!(
        total_packets = stats.total_packets,
        allowed_packets = stats.allowed_packets,
        blocked_packets = stats.blocked_packets,
        active_flows = fw.active_flows(),
        "Firewall stats"
    );

    info!("Daemon running. Send SIGINT to stop.");
    if let Err(err) = tokio::signal::ctrl_c().await {
        warn!(?err, "ctrl_c handler failed");
    }
    info!("Shutting down");
}
