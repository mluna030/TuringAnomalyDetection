use std::net::IpAddr;
use crate::domain::packet::Packet;
use crate::domain::rule::{Filter, Action};

pub struct IpRules {
    pub ip_addr: IpAddr,
}

impl Filter for IpRules {
    fn check_packet (
        &self, 
        packet: &Packet
    ) -> Option<Action> {
        if packet.source_ip == self.ip_addr {
            Option::Some(Action::Block)
        } else {
            Option::None
        }
    }
} 
