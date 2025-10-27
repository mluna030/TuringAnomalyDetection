use crate::traits::{Filter, Action};
use crate::packets::Packet;

pub struct GeoRules {
    pub country_codes: Vec<String>,
}

impl Filter for GeoRules {
    fn check_packet(
        &self, 
        packet: &Packet
    ) -> Option<Action> {
        for codes in country_codes {
            if packet.source_ip == self.country_codes {
                Option::Some(Action::Block);
            } else {
                Option::None;
            }
        }
    }
}