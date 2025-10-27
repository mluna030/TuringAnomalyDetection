use crate::packet::{Packet, PacketHeader, FlowKey};
use crate::traits::{Action, Filter};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct FirewallEngine {
    rules: Vec<FilterEntry>,                            // Rule Chain - Sorted by priority
    default_action: Action,                             // Default Action when no rules match
    flow_table: HashMap<FlowKey, FlowStats>,            // Flow tracking for stateful filtering
    stats: FirewallStats,                               // Statistics
}

struct FilterEntry {
    id: u64,
    filter: Box<dyn Filter>,
    enabled: bool,
    hit_count: u64,
}



pub struct FirewallStats {
    pub total_packets: u64,
    pub allowed_packets: u64,
    pub blocked_packets: u64,
    pub packets_per_second: u64,
}