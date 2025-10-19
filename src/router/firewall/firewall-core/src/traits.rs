use crate::packet::Packet;

// Action taken after a filter rules is match 
pub enum Action {
    Allow,
    Block,
    Log,
}

pub trait Filter {
    fn check_packet(
        &self, 
        packet: &Packet
    ) -> Option<Action>;
}
