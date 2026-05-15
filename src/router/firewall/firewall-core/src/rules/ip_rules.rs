use crate::domain::packet::{Packet, PacketHeader};
use crate::domain::rule::{Action, Filter};
use std::collections::HashSet;
use std::net::IpAddr;

pub struct IpBlocklistRule {
    name: String,
    blocked: HashSet<IpAddr>,
    action: Action,
    priority: i32,
    match_destination: bool,
}

impl IpBlocklistRule {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            blocked: HashSet::new(),
            action: Action::Block,
            priority: 90,
            match_destination: false,
        }
    }

    pub fn add_ip(mut self, ip: IpAddr) -> Self {
        self.blocked.insert(ip);
        self
    }

    pub fn add_ips(mut self, ips: impl IntoIterator<Item = IpAddr>) -> Self {
        self.blocked.extend(ips);
        self
    }

    pub fn match_destination(mut self, enabled: bool) -> Self {
        self.match_destination = enabled;
        self
    }

    pub fn with_action(mut self, action: Action) -> Self {
        self.action = action;
        self
    }

    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
}

impl Filter for IpBlocklistRule {
    fn quick_match(&self, _header: &PacketHeader) -> bool {
        !self.blocked.is_empty()
    }

    fn check_packet(&self, packet: &Packet) -> Option<Action> {
        if self.blocked.contains(&packet.source_ip) {
            return Some(self.action);
        }
        if self.match_destination && self.blocked.contains(&packet.destination_ip) {
            return Some(self.action);
        }
        None
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn priority(&self) -> i32 {
        self.priority
    }
}
