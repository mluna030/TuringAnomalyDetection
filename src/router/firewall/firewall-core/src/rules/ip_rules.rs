use std::net::IpAddr;
use crate::traits::{Filter, Action};
use crate::packet::Packet;

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
