use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct Packet {
    pub source_ip: IpAddr,
    pub destination_ip: IpAddr,
    pub source_port: Option<u16>,
    pub destination_port: Option<u16>,
    pub protocol: Protocol,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
pub struct PacketHeader {
    pub source_ip: IpAddr,
    pub destination_ip: IpAddr,
    pub source_port: Option<u16>,
    pub destination_port: Option<u16>,
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
    pub fn to_number(self) -> u8 {
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
            source_port: None,
            destination_port: None,
            protocol: Protocol::Unknown,
            payload: Vec::new(),
        }
    }

    pub fn with_destination(mut self, ip: IpAddr) -> Self {
        self.destination_ip = ip;
        self
    }

    pub fn with_ports(mut self, src: u16, dst: u16) -> Self {
        self.source_port = Some(src);
        self.destination_port = Some(dst);
        self
    }

    pub fn with_protocol(mut self, protocol: Protocol) -> Self {
        self.protocol = protocol;
        self
    }

    pub fn with_payload(mut self, payload: Vec<u8>) -> Self {
        self.payload = payload;
        self
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
