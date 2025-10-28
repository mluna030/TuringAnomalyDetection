use crate::rules::time_rules::TimeWindow;
use crate::domain::rate_limiter::TokenBucket;
use std::collections::HashMap;
use std::net::IpAddr;

pub struct TimeBasedRateLimitRule {
    name: String,
    limits: Vec<(TimeWindow, f64)>,
    default_rate: f64,
    buckets: Arc<Mutex<HashMap<IpAddr, TokenBucket>>>,
    priority: i32,
}

impl TimeBasedRateLimitRule {
    pub fn new(name: impl Into<String>, default_rate: f64) -> Self {
        Self {
            name: name.into(),
            limits: Vec::new(),
            default_rate,
            buckets: Arc::new(Mutex::new(HashMap::new())),
            priority: 65,
        }
    }
    pub fn add_time_limit(mut self, window: TimeWindow, rate: f64) -> Self {
        self.limits.push((window, rate));
        self
    }
    
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
    fn current_rate(&self) -> f64 {
        for (window, rate) in &self.limits {
            if window.is_now_in_window() {
                return *rate;
            }
        }
        self.default_rate
    }
    pub fn cleanup(&self, threshold_secs: u64) {
        let mut buckets = self.buckets.lock().unwrap();
        let now = std::time::Instant::now();
        
        buckets.retain(|_, bucket| {
            now.duration_since(bucket.last_refill).as_secs() < threshold_secs
        });
    }
}

impl Filter for TimeBasedRateLimitRule {
    fn quick_match(&self, _header: &PacketHeader) -> bool {
        true
    }
    
    fn check_packet(&self, packet: &Packet) -> Option<Action> {
        let source_ip = packet.source_ip;
        let current_rate = self.current_rate();
        
        let mut buckets = self.buckets.lock().unwrap();
        let bucket = buckets.entry(source_ip).or_insert_with(|| {
            TokenBucket::new(current_rate, current_rate)
        });
        bucket.set_rate(current_rate);
        
        if bucket.try_consume(1.0) {
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