use crate::domain::packet::{Packet, PacketHeader};

pub enum Action {
    Allow,
    Block,
    Log,
}

pub trait Filter: Send + Sync {
    fn quick_match(&self, header: &PacketHeader) -> bool {
        true
    }
    fn check_packet(&self, packet: &Packet) -> Option<Action>;
    fn name(&self) -> &str {
        "UnnamedFilter"
    }
    fn priority(&self) -> i32 {
        0
    }
}

pub struct RuleEntry {
    pub id: u64,
    pub filter: Box<dyn Filter>,
    pub enabled: bool,
    pub hit_count: u64,
}