use crate::domain::{
    packet::{Packet, PacketHeader},
    rule::{Action, Filter, RuleEntry},
    flow::{FlowKey, FlowTracker},
    stats::StatsCollector,
};
use std::sync::{Arc, RwLock};

pub struct PacketProcessor {
    rules: Arc<RwLock<Vec<RuleEntry>>>,
    default_action: Action,
    flow_tracker: Arc<FlowTracker>,
    stats_collector: Arc<dyn StatsCollector>,
}

impl PacketProcessor {
    pub fn new(
        default_action: Action,
        flow_tracker: Arc<FlowTracker>,
        stats_collector: Arc<dyn StatsCollector>,
        rules: Arc<RwLock<Vec<RuleEntry>>>,
    ) -> Self {
        Self {
            rules,
            default_action,
            flow_tracker,
            stats_collector,
        }
    }

    pub fn process(&self, packet: &Packet) -> Action {
        let flow_key = FlowKey::new(
          packet.source_ip,
          packet.destination_ip,
          packet.source_port,
          packet.destination_port,
          packet.protocol.to_number(),
        );

        self.flow_tracker.record_packet(flow_key, packet.payload.len());
        // Checks rules
        let action = self.evaluate_rules(packet);
        // Records Statistics
        self.stats_collector.record_packet(&action);

        action
    }

    fn evaluate_rules(&self, packet: &Packet) -> Action {
        let rules = self.rules.read().unwrap();
        let header = packet.header();

        for entry in rules.iter() {
            if !entry.enabled {
                continue;
            }

            if !entry.filter.quick_match(&header) {
                continue;
            }

            if let Some(action) = entry.filter.check_packet(packet) {
                return action;
            }
        }
        self.default_action;
    }
    pub(crate) fn rules(&self) -> Arc<RwLock<Vec<RuleEntry>>> {
        Arc::clone(&self.rules)
    }
}