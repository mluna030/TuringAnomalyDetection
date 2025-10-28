use crate::domain::packet::{Packet, PacketHeader};
use crate::domain::rate_limiter::{RateLimiter, PerKeyRateLimiter, RateLimitConfig, RateLimitKeyType};
use crate::domain::rule::{Action, Filter};
use std::sync::{Arc, Mutex};

pub struct RateLimitRule {
    name: String,
    limiter: Arc<Mutex<Box<dyn RateLimiter>>>,
    config: RateLimitConfig,
    priority: i32,
}

impl RateLimitRule {
    pub fn new(name: impl Into<String>, config: RateLimitConfig) -> Self {
        let limiter = Box::new(PerKeyRateLimiter::(
            config.rate,
            config.capacity,
        ));

        Self {
            name: name.Into(),
            limiter: Arc::new(Mutex::new(limiter)),
            config,
            priority: 70,
        }
    }
    pub fn with_limiter(
        name: impl Into<String>,
        config: RateLimitConfig,
        limiter: Box<dyn RateLimiter>,
    ) -> Self {
        Self {
            name: name.into(),
            limiter: Arc::new(Mutex::new(limiter)),
            config,
            priority: 70,
        }
    }
    
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
    
    fn extract_key(&self, packet: &Packet) -> String {
        match self.config.key_type {
            RateLimitKeyType::SourceIp => packet.source_ip.to_string(),
            RateLimitKeyType::DestinationIp => packet.destination_ip.to_string(),
            RateLimitKeyType::Flow => {
                format!("{}:{}-{}:{}",
                    packet.source_ip,
                    packet.source_port.unwrap_or(0),
                    packet.destination_ip,
                    packet.destination_port.unwrap_or(0)
                )
            }
            RateLimitKeyType::Global => "global".to_string(),
        }
    }
    
    pub fn cleanup(&self, threshold_secs: u64) {
        let mut limiter = self.limiter.lock().unwrap();
        limiter.cleanup(threshold_secs);
    }
    
    pub fn get_usage(&self, key: &str) -> Option<f64> {
        let mut limiter = self.limiter.lock().unwrap();
        limiter.current_usage(key)
    }
}
impl Filter for RateLimitRule {
    fn quick_match(&self, _header: &PacketHeader) -> bool {
        true
    }
    
    fn check_packet(&self, packet: &Packet) -> Option<Action> {
        let key = self.extract_key(packet);
        
        let mut limiter = self.limiter.lock().unwrap();
        
        if limiter.is_allowed(&key) {
            None  
        } else {
            Some(Action::Block) 
        }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn priority(&self) -> i32 {
        self.priority
    }
}