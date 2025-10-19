use std::net::IpAddr;

pub struct Packet {
    pub source_ip: IpAddr,
    pub destination_ip: IpAddr,
    pub source_port: u16,
    pub destination_port: u16,
    pub protocol: Protocol,
    pub payload: Vec<u8>,
}

pub struct Protocol {
    Tcp,
    Udp,
    Icmp,
    Unknown,
}
