use crate::domain::flow::{FlowKey, FlowStats, FlowTracker};
use crate::domain::stats::{FirewallStats, StatsCollector};
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
pub struct MqttPublisherConfig {
    pub broker_host: String,
    pub broker_port: u16,
    pub client_id: String,
    pub flow_topic: String,
    pub stats_topic: String,
    pub interval: Duration,
    pub keep_alive: Duration,
}

impl Default for MqttPublisherConfig {
    fn default() -> Self {
        Self {
            broker_host: "127.0.0.1".to_string(),
            broker_port: 1883,
            client_id: "firewall-router".to_string(),
            flow_topic: "turing/router/flows".to_string(),
            stats_topic: "turing/router/stats".to_string(),
            interval: Duration::from_secs(5),
            keep_alive: Duration::from_secs(15),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlowReport {
    pub src_ip: String,
    pub dest_ip: String,
    pub src_port: Option<u16>,
    pub dest_port: Option<u16>,
    pub protocol: u8,
    pub packets: u64,
    pub bytes: u64,
    pub age_secs: u64,
}

impl FlowReport {
    pub fn from_flow(key: &FlowKey, stats: &FlowStats) -> Self {
        Self {
            src_ip: key.src_ip.to_string(),
            dest_ip: key.dest_ip.to_string(),
            src_port: key.src_port,
            dest_port: key.dest_port,
            protocol: key.protocol,
            packets: stats.packets,
            bytes: stats.bytes,
            age_secs: stats.last_seen.duration_since(stats.first_seen).as_secs(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatsReport {
    pub total_packets: u64,
    pub allowed_packets: u64,
    pub blocked_packets: u64,
    pub inspected_packets: u64,
    pub packets_per_second: f64,
    pub active_flows: usize,
}

impl StatsReport {
    pub fn build(stats: &FirewallStats, active_flows: usize) -> Self {
        Self {
            total_packets: stats.total_packets,
            allowed_packets: stats.allowed_packets,
            blocked_packets: stats.blocked_packets,
            inspected_packets: stats.inspected_packets,
            packets_per_second: stats.packets_per_second,
            active_flows,
        }
    }
}

pub struct MqttPublisher {
    client: AsyncClient,
    config: MqttPublisherConfig,
}

impl MqttPublisher {
    pub fn new(config: MqttPublisherConfig) -> (Self, EventLoop) {
        let mut opts =
            MqttOptions::new(&config.client_id, &config.broker_host, config.broker_port);
        opts.set_keep_alive(config.keep_alive);
        let (client, eventloop) = AsyncClient::new(opts, 32);
        (Self { client, config }, eventloop)
    }

    pub fn client(&self) -> AsyncClient {
        self.client.clone()
    }

    pub async fn publish_flow(&self, report: &FlowReport) -> Result<(), String> {
        let payload = serde_json::to_vec(report).map_err(|e| e.to_string())?;
        self.client
            .publish(&self.config.flow_topic, QoS::AtLeastOnce, false, payload)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn publish_stats(&self, report: &StatsReport) -> Result<(), String> {
        let payload = serde_json::to_vec(report).map_err(|e| e.to_string())?;
        self.client
            .publish(&self.config.stats_topic, QoS::AtLeastOnce, false, payload)
            .await
            .map_err(|e| e.to_string())
    }

    pub fn spawn(
        self,
        mut eventloop: EventLoop,
        flow_tracker: Arc<FlowTracker>,
        stats_collector: Arc<dyn StatsCollector>,
    ) -> (JoinHandle<()>, JoinHandle<()>) {
        let interval = self.config.interval;
        let publisher = Arc::new(self);

        let event_handle = tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("mqtt event loop error: {err}");
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        });

        let publish_handle = tokio::spawn(async move {
            let mut ticker = tokio::time::interval(interval);
            loop {
                ticker.tick().await;
                let snapshot = flow_tracker.snapshot();
                let active = snapshot.len();
                for (key, stats) in &snapshot {
                    let report = FlowReport::from_flow(key, stats);
                    if let Err(err) = publisher.publish_flow(&report).await {
                        eprintln!("mqtt publish flow failed: {err}");
                    }
                }
                let stats_report = StatsReport::build(&stats_collector.get_stats(), active);
                if let Err(err) = publisher.publish_stats(&stats_report).await {
                    eprintln!("mqtt publish stats failed: {err}");
                }
            }
        });

        (event_handle, publish_handle)
    }
}
