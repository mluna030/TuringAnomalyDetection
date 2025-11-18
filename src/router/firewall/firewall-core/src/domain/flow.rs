use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlowKey {
    pub src_ip: IpAddr,
    pub dest_ip: IpAddr,
    pub src_port: Option<u16>,
    pub dest_port: Option<u16>,
    pub protocol: u8,
}

impl FlowKey {
    pub fn new(
        src_ip: IpAddr,
        dest_ip: IpAddr,
        src_port: Option<u16>,
        dest_port: Option<u16>,
        protocol: u8,
    ) -> Self {
        FlowKey {
            src_ip,
            dest_ip,
            src_port,
            dest_port,
            protocol,
        }
    }

    pub fn reverse (&self) -> Self {
        Self {
            src_ip: self.dest_ip,
            dest_ip: self.src_ip,
            src_port: self.dest_port,  
            dest_port: self.src_port,
            protocol: self.protocol,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FlowStats {
    pub packets: u64,
    pub bytes: u64,
    pub first_seen: std::time::Instant,
    pub last_seen: std::time::Instant
}

//Statistics for a network flow
impl FlowStats {
    pub fn new() -> Self {
        let now = std::time::Instant::now();
        FlowStats {
            packets: 0,
            bytes: 0,
            first_seen: now,
            last_seen: now,
        }
    }
    pub fn update(&mut self, bytes: usize) {
        self.packets += 1;
        self.bytes += bytes as u64;
        self.last_seen = std::time::Instant::now();
    }
}

// manages network flow tracking
pub struct FlowTracker {
    flows: Arc<Mutex<HashMap<FlowKey, FlowStats>>>,
}

impl FlowTracker {
    pub fn new() -> Self {
        Self {
            flows: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn record_packet(&self, flow_key: FlowKey, bytes: usize) {
        let flows = self.flows.lock().unwrap();
        flows.entry(flow_key)
            .or_insert(FlowStats::new)
            .update(bytes);
    }

    pub fn get_flow(&self, flow_key: &FlowKey) -> Option<FlowStats> {
        let flows = self.flows.lock().unwrap();
        flows.get(flow_key).cloned()
    }

    pub fn cleanup_old_flows(&self, max_age_secs: u64) {
        let mut flows = self.flows.lock().unwrap();
        let now = std::time::Instant::now();

        flows.retain(|_, stats| {
            now.duration_since(stats.last_seen).as_secs() < max_age_secs
        });
    }

    pub fn active_flow_count(&self) -> usize {
        let flows = self.flows.lock().unwrap();
        flows.len()
    }
}