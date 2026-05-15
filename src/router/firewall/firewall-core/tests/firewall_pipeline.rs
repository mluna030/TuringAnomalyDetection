use firewall_core::domain::packet::Protocol;
use firewall_core::domain::rate_limiter::{RateLimitConfig, RateLimitKeyType};
use firewall_core::rules::ip_rules::IpBlocklistRule;
use firewall_core::rules::port_rules::{PortBlocklistRule, Service, WellKnownServicesRule};
use firewall_core::rules::rate_limit_rules::rate_limit_rules::RateLimitRule;
use firewall_core::{Action, FirewallBuilder, Packet};
use std::net::{IpAddr, Ipv4Addr};

fn pkt(src_octet: u8, proto: Protocol, dport: u16) -> Packet {
    Packet::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, src_octet)))
        .with_destination(IpAddr::V4(Ipv4Addr::new(10, 0, 1, 1)))
        .with_ports(40000, dport)
        .with_protocol(proto)
        .with_payload(vec![0u8; 64])
}

#[test]
fn default_block_drops_unmatched_traffic() {
    let fw = FirewallBuilder::new(Action::Block).build();
    let action = fw.process_packet(&pkt(7, Protocol::Tcp, 9999));
    assert_eq!(action, Action::Block);

    let stats = fw.get_stats();
    assert_eq!(stats.total_packets, 1);
    assert_eq!(stats.blocked_packets, 1);
    assert_eq!(stats.allowed_packets, 0);
}

#[test]
fn allow_by_default_with_admin_port_block() {
    let fw = FirewallBuilder::new(Action::Allow).build();
    fw.add_rule(Box::new(
        PortBlocklistRule::new("admin").add_port(Service::Telnet.port()),
    ));

    let allowed = fw.process_packet(&pkt(1, Protocol::Tcp, Service::Https.port()));
    let blocked = fw.process_packet(&pkt(2, Protocol::Tcp, Service::Telnet.port()));

    assert_eq!(allowed, Action::Allow);
    assert_eq!(blocked, Action::Block);

    let stats = fw.get_stats();
    assert_eq!(stats.total_packets, 2);
    assert_eq!(stats.allowed_packets, 1);
    assert_eq!(stats.blocked_packets, 1);
}

#[test]
fn ip_blocklist_short_circuits_pipeline() {
    let fw = FirewallBuilder::new(Action::Allow).build();
    let bad_ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 42));
    fw.add_rule(Box::new(
        IpBlocklistRule::new("bad-src").add_ip(bad_ip).with_priority(100),
    ));

    let blocked = fw.process_packet(&pkt(42, Protocol::Tcp, Service::Https.port()));
    let allowed = fw.process_packet(&pkt(43, Protocol::Tcp, Service::Https.port()));

    assert_eq!(blocked, Action::Block);
    assert_eq!(allowed, Action::Allow);
}

#[test]
fn rate_limit_blocks_after_burst() {
    let fw = FirewallBuilder::new(Action::Allow).build();
    let cfg = RateLimitConfig {
        rate: 0.0,
        capacity: 3.0,
        key_type: RateLimitKeyType::SourceIp,
    };
    fw.add_rule(Box::new(RateLimitRule::new("burst", cfg)));

    let mut allowed = 0u64;
    let mut blocked = 0u64;
    for _ in 0..10 {
        match fw.process_packet(&pkt(5, Protocol::Tcp, Service::Https.port())) {
            Action::Allow | Action::Log => allowed += 1,
            Action::Block => blocked += 1,
        }
    }
    assert_eq!(allowed, 3, "first capacity packets must pass");
    assert_eq!(blocked, 7, "remaining packets must be dropped");
}

#[test]
fn well_known_services_allowlist_path() {
    let fw = FirewallBuilder::new(Action::Block).build();
    fw.add_rule(Box::new(
        WellKnownServicesRule::new("iot").allow_iot_services(),
    ));

    let mqtt = fw.process_packet(&pkt(1, Protocol::Tcp, Service::Mqtt.port()));
    let https = fw.process_packet(&pkt(2, Protocol::Tcp, Service::Https.port()));
    let telnet = fw.process_packet(&pkt(3, Protocol::Tcp, Service::Telnet.port()));

    assert_eq!(mqtt, Action::Allow);
    assert_eq!(https, Action::Allow);
    assert_eq!(telnet, Action::Block);
}

#[test]
fn flow_tracker_records_packet_counts() {
    let fw = FirewallBuilder::new(Action::Allow).build();
    for _ in 0..5 {
        fw.process_packet(&pkt(1, Protocol::Tcp, Service::Https.port()));
    }
    fw.process_packet(&pkt(2, Protocol::Tcp, Service::Https.port()));
    assert_eq!(fw.active_flows(), 2);
}
