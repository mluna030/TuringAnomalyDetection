use std::net::IpAddr;
use crate::Protocol;

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
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
    Unknown,
}

impl Packet {
    pub fn new(&self, source_ip: IpAddr) -> Self {
        Packet {
            source_ip,
            destination_ip: IpAddr::default(),
            source_port: 0,
            destination_port: 0,
            protocol: Protocol,
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


