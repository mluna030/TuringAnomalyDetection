use std::sync::Arc;

// Domain Layer: Core Business Layer
pub mod domain {
    pub mod packet;
    pub mod flow;
    pub mod rule;
    pub mod stats;
}

//Application Layer: Use cases
pub mod Application {
    pub mod engine;
    pub mod rule_manager;
}

// Infrastructure Layer: External Integrations
pub mod  Infrastructure {
    pub mod backends;
    pub mod mqtt;
}

// Rules: Filter Trait
pub mod rules {
    pub mod ip_rules;
    pub mod port_rules;
    pub mod geo_rules;
    pub mod time_rules;
}

pub struct Firewall {
    processor: Arc<PacketProcessor>,
    rule_manager: Arc<RuleManager>,
    flow_tracker: Arc<FlowTracker>,
    stats_collector: Arc<dyn StatsCollector>,
}

impl Firewall {
    pub fn new(default_action: Action) -> Self {
        let flow_tracker = Arc::new(FlowTracker::new());
        let stats_collector: Arc<dyn StatsCollector> = Arc::new(InMemoryStatsCollector::new());
        let rule_manager = Arc::new(RuleManager::new());

        let processor = Arc::new(PacketProcessor::new(
            default_action,
            Arc::clone(&flow_tracker),
            Arc::clone(&stats_collector),
            rule_manager.rules_ref(),
        ));
        Self {
            processor,
            rule_manager,
            flow_tracker,
            stats_collector,
        }
    }

    pub fn process_packet(&self, packet: &Packet) -> Action {
        self.processor.process(packet)
    }
    pub fn add_rule(&self, filter: Box<dyn Filter>) -> u64 {
        self.rule_manager.add_rule(filter)
    }
    pub fn remove_rule(&self, id: u64) -> bool {
        self.rule_manager.remove_rule(id)
    }
    pub fn set_rule_enabled(&self, id: u64, enabled: bool) -> bool {
        self.rule_manager.set_enabled(id, enabled)
    }
    pub fn list_rules(&self) -> Vec<RuleInfo> {
        self.rule_manager.list_rules()
    }

    pub fn get_stats(&self) -> FirewallStats {
        self.stats_collector.get_stats()
    }

    pub fn active_flows(&self) -> usize {
        self.flow_tracker.active_flow_count()
    }

    pub fn cleanup_old_flows(&self, max_age_secs: u64) {
        self.flow_tracker.cleanup_old_flows(max_age_secs)
    }

}
pub struct FirewallBuilder {
    default_action: Action,
    stats_collector: Option<Arc<dyn StatsCollector>>,
    // Could add more options later:
    // max_flows: Option<usize>,
    // flow_timeout: Option<u64>,
}

impl FirewallBuilder {
    pub fn new(default_action: Action) -> Self {
        Self {
            default_action,
            stats_collector: None,
        }
    }

    pub fn with_stats_collector(mut self, collector: Arc<dyn StatsCollector>) -> Self {
        self.stats_collector = Some(collector);
        self
    }

    pub fn build(self) -> Firewall {
        let flow_tracker = Arc::new(FlowTracker::new());

        // Use custom or default stats collector
        let stats_collector = self.stats_collector
            .unwrap_or_else(|| Arc::new(InMemoryStatsCollector::new()));

        let rule_manager = Arc::new(RuleManager::new());

        let processor = Arc::new(PacketProcessor::new(
            self.default_action,
            Arc::clone(&flow_tracker),
            Arc::clone(&stats_collector),
            rule_manager.rules_ref(),
        ));

        Firewall {
            processor,
            rule_manager,
            flow_tracker,
            stats_collector,
        }
    }
}
// ReExports
pub use domain::packet::{Packet, Protocol, PacketHeader};
pub use domain::rule::{Filter, Action, RuleEntry};
pub use domain::flow::{FlowKey, FlowStats, FlowTracker};
pub use domain::stats::{FirewallStats, StatsCollector, InMemoryStatsCollector};
pub use application::engine::PacketProcessor;
pub use application::rule_manager::{RuleManager, RuleInfo};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");