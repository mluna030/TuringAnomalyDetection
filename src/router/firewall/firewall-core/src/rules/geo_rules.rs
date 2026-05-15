use crate::domain::packet::{Packet, PacketHeader};
use crate::domain::rule::{Action, Filter};
use std::net::IpAddr;

pub struct GeoBlockRule {
    name: String,
    blocked_ranges: Vec<IpRange>,
    action: Action,
    priority: i32,
}

#[derive(Debug, Clone)]
pub struct IpRange {
    pub start: IpAddr,
    pub end: IpAddr,
    pub country_code: String,
}

impl IpRange {
    pub fn contains(&self, ip: &IpAddr) -> bool {
        match (self.start, self.end, ip) {
            (IpAddr::V4(s), IpAddr::V4(e), IpAddr::V4(target)) => {
                let s = u32::from(s);
                let e = u32::from(e);
                let t = u32::from(*target);
                t >= s && t <= e
            }
            (IpAddr::V6(s), IpAddr::V6(e), IpAddr::V6(target)) => {
                let s = u128::from(s);
                let e = u128::from(e);
                let t = u128::from(*target);
                t >= s && t <= e
            }
            _ => false,
        }
    }
}

impl GeoBlockRule {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            blocked_ranges: Vec::new(),
            action: Action::Block,
            priority: 70,
        }
    }

    pub fn add_range(mut self, start: IpAddr, end: IpAddr, country_code: impl Into<String>) -> Self {
        self.blocked_ranges.push(IpRange {
            start,
            end,
            country_code: country_code.into(),
        });
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

impl Filter for GeoBlockRule {
    fn check_packet(&self, packet: &Packet) -> Option<Action> {
        for range in &self.blocked_ranges {
            if range.contains(&packet.source_ip) {
                return Some(self.action);
            }
        }
        None
    }

    fn quick_match(&self, _header: &PacketHeader) -> bool {
        !self.blocked_ranges.is_empty()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn priority(&self) -> i32 {
        self.priority
    }
}
