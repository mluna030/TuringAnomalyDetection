use crate::domain::packet::{Packet, PacketHeader, Protocol};
use crate::domain::rule::{Action, Filter};
use std::collections::HashSet;
use std::ops::RangeInclusive;

pub struct PortBlocklistRule {
    name: String,
    blocked_ports: HashSet<u16>,
    blocked_ranges: Vec<RangeInclusive<u16>>,
    protocols: HashSet<Protocol>,
    action: Action,
    priority: i32,
    match_source: bool,
    match_destination: bool,
}

impl PortBlocklistRule {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            blocked_ports: HashSet::new(),
            blocked_ranges: Vec::new(),
            protocols: HashSet::new(),
            action: Action::Block,
            priority: 80,
            match_source: false,
            match_destination: true,
        }
    }

    pub fn add_port(mut self, port: u16) -> Self {
        self.blocked_ports.insert(port);
        self
    }

    pub fn add_ports(mut self, ports: impl IntoIterator<Item = u16>) -> Self {
        self.blocked_ports.extend(ports);
        self
    }

    pub fn add_range(mut self, start: u16, end: u16) -> Self {
        self.blocked_ranges.push(start..=end);
        self
    }

    pub fn for_protocol(mut self, protocol: Protocol) -> Self {
        self.protocols.insert(protocol);
        self
    }

    pub fn tcp_only(self) -> Self {
        self.for_protocol(Protocol::Tcp)
    }

    pub fn udp_only(self) -> Self {
        self.for_protocol(Protocol::Udp)
    }

    pub fn match_source(mut self, enabled: bool) -> Self {
        self.match_source = enabled;
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

    fn is_port_blocked(&self, port: u16) -> bool {
        if self.blocked_ports.contains(&port) {
            return true;
        }

        for range in &self.blocked_ranges {
            if range.contains(&port) {
                return true;
            }
        }

        false
    }

    fn matches_protocol(&self, protocol: &Protocol) -> bool {
        self.protocols.is_empty() || self.protocols.contains(protocol)
    }
}

impl Filter for PortBlocklistRule {
    fn quick_match(&self, header: &PacketHeader) -> bool {
        if !self.matches_protocol(&header.protocol) {
            return false;
        }

        if self.match_source && header.source_port.is_some() {
            return true;
        }
        if self.match_destination && header.destination_port.is_some() {
            return true;
        }

        false
    }

    fn check_packet(&self, packet: &Packet) -> Option<Action> {
        if !self.matches_protocol(&packet.protocol) {
            return None;
        }

        let mut matched = false;

        if self.match_source {
            if let Some(port) = packet.source_port {
                if self.is_port_blocked(port) {
                    matched = true;
                }
            }
        }

        if self.match_destination {
            if let Some(port) = packet.destination_port {
                if self.is_port_blocked(port) {
                    matched = true;
                }
            }
        }

        if matched {
            Some(self.action)
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn priority(&self) -> i32 {
        self.priority
    }
}

pub struct PortAllowlistRule {
    name: String,
    allowed_ports: HashSet<u16>,
    allowed_ranges: Vec<RangeInclusive<u16>>,
    protocols: HashSet<Protocol>,
    priority: i32,
    match_destination: bool,
}

impl PortAllowlistRule {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            allowed_ports: HashSet::new(),
            allowed_ranges: Vec::new(),
            protocols: HashSet::new(),
            priority: 90,
            match_destination: true,
        }
    }

    pub fn add_port(mut self, port: u16) -> Self {
        self.allowed_ports.insert(port);
        self
    }

    pub fn add_ports(mut self, ports: impl IntoIterator<Item = u16>) -> Self {
        self.allowed_ports.extend(ports);
        self
    }

    pub fn add_range(mut self, start: u16, end: u16) -> Self {
        self.allowed_ranges.push(start..=end);
        self
    }

    pub fn for_protocol(mut self, protocol: Protocol) -> Self {
        self.protocols.insert(protocol);
        self
    }

    pub fn tcp_only(self) -> Self {
        self.for_protocol(Protocol::Tcp)
    }

    pub fn udp_only(self) -> Self {
        self.for_protocol(Protocol::Udp)
    }

    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    fn is_port_allowed(&self, port: u16) -> bool {
        if self.allowed_ports.contains(&port) {
            return true;
        }

        for range in &self.allowed_ranges {
            if range.contains(&port) {
                return true;
            }
        }

        false
    }

    fn matches_protocol(&self, protocol: &Protocol) -> bool {
        self.protocols.is_empty() || self.protocols.contains(protocol)
    }
}

impl Filter for PortAllowlistRule {
    fn quick_match(&self, header: &PacketHeader) -> bool {
        self.matches_protocol(&header.protocol) && header.destination_port.is_some()
    }

    fn check_packet(&self, packet: &Packet) -> Option<Action> {
        if !self.matches_protocol(&packet.protocol) {
            return None;
        }

        if !self.match_destination {
            return None;
        }

        if let Some(port) = packet.destination_port {
            if !self.is_port_allowed(port) {
                return Some(Action::Block);
            }
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Service {
    Http,       // TCP 80
    Https,      // TCP 443
    Ssh,        // TCP 22
    Telnet,     // TCP 23
    Ftp,        // TCP 21
    Smtp,       // TCP 25
    Dns,        // TCP/UDP 53
    Dhcp,       // UDP 67/68
    Mqtt,       // TCP 1883
    MqttTls,    // TCP 8883
    Rdp,        // TCP 3389
}

impl Service {
    pub fn port(&self) -> u16 {
        match self {
            Service::Http => 80,
            Service::Https => 443,
            Service::Ssh => 22,
            Service::Telnet => 23,
            Service::Ftp => 21,
            Service::Smtp => 25,
            Service::Dns => 53,
            Service::Dhcp => 67,
            Service::Mqtt => 1883,
            Service::MqttTls => 8883,
            Service::Rdp => 3389,
        }
    }

    pub fn protocol(&self) -> Protocol {
        match self {
            Service::Dhcp => Protocol::Udp,
            Service::Dns => Protocol::Udp,
            _ => Protocol::Tcp,
        }
    }

    pub fn service_name(&self) -> &str {
        match self {
            Service::Http => "HTTP",
            Service::Https => "HTTPS",
            Service::Ssh => "SSH",
            Service::Telnet => "Telnet",
            Service::Ftp => "FTP",
            Service::Smtp => "SMTP",
            Service::Dns => "DNS",
            Service::Dhcp => "DHCP",
            Service::Mqtt => "MQTT",
            Service::MqttTls => "MQTT/TLS",
            Service::Rdp => "RDP",
        }
    }
}

pub struct WellKnownServicesRule {
    name: String,
    services: HashSet<Service>,
    action: Action,
    priority: i32,
}

impl WellKnownServicesRule {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            services: HashSet::new(),
            action: Action::Block,
            priority: 75,
        }
    }

    pub fn add_service(mut self, service: Service) -> Self {
        self.services.insert(service);
        self
    }

    pub fn add_services(mut self, services: impl IntoIterator<Item = Service>) -> Self {
        self.services.extend(services);
        self
    }
    pub fn block_dangerous_services(self) -> Self {
        self.add_services(vec![
            Service::Telnet,  // Unencrypted
            Service::Ftp,     // Unencrypted
            Service::Rdp,     // Often attacked
        ])
    }

    pub fn allow_iot_services(self) -> Self {
        self.add_services(vec![
            Service::Http,
            Service::Https,
            Service::Mqtt,
            Service::MqttTls,
            Service::Dns,
        ])
            .with_action(Action::Allow)
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

impl Filter for WellKnownServicesRule {
    fn quick_match(&self, header: &PacketHeader) -> bool {
        header.destination_port.is_some()
    }

    fn check_packet(&self, packet: &Packet) -> Option<Action> {
        if let Some(dst_port) = packet.destination_port {
            for service in &self.services {
                if dst_port == service.port() && packet.protocol == service.protocol() {
                    return Some(self.action);
                }
            }
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