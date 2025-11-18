use std::net::IpAddr;

pub struct Packet {
    pub source_ip: IpAddr,
    pub destination_ip: IpAddr,
    pub source_port: u16,
    pub destination_port: u16,
    pub protocol: Protocol,
    pub payload: Vec<u8>,
}
pub struct PacketHeader {
    pub source_ip: IpAddr,
    pub destination_ip: IpAddr,
    pub source_port: u16,
    pub destination_port: u16,
    pub protocol: Protocol,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
    Unknown,
}
impl Protocol {
    pub fn to_number(&self) -> u8 {
        match self {
            Protocol::Tcp => 6,
            Protocol::Udp => 17,
            Protocol::Icmp => 1,
            Protocol::Unknown => 0,
        }
    }
}
impl Packet {
    pub fn new(source_ip: IpAddr) -> Self {
        Packet {
            source_ip,
            destination_ip: IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED),
            source_port: 0,
            destination_port: 0,
            protocol: Protocol::Unknown,
            payload: Vec::new(),
        }
    }
    pub fn header(&self) -> PacketHeader {
        PacketHeader {
            source_ip: self.source_ip,
            destination_ip: self.destination_ip,
            source_port: self.source_port,
            destination_port: self.destination_port,
            protocol: self.protocol,
        }
    }
}


