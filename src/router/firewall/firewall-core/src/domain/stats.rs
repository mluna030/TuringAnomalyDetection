use domain::rule::Action;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct FirewallStats {
    pub total_packets: u64,
    pub allowed_packets: u64,
    pub blocked_packets: u64,
    pub inspected_packets: u64,
    pub packets_per_second: f64,
    start_time: std::time::Instant,
}

impl FirewallStats {
    pub fn new() -> Self {
        Self {
            total_packets: 0,
            allowed_packets: 0,
            blocked_packets: 0,
            inspected_packets: 0,
            packets_per_second: 0.0,
            start_time: std::time::Instant::now(),
        }
    }
}

pub trait StatsCollector: Send + Sync {
    fn record_packet(&self, action: &Action);
    fn get_stats(&self) -> FirewallStats;
    fn reset(&self);
}

pub struct InMemoryStatsCollector {
    stats: Arc<Mutex<FirewallStats>>,
}

impl InMemoryStatsCollector {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(FirewallStats::new())),
        }
    }
}

impl StatsCollector for InMemoryStatsCollector {
    fn record_packet(&self, action: &Action) {
        let mut stats = self.stats.lock().unwrap();
        stats.total_packets += 1;

        match action {
            Action::Allow | Action::Log => stats.allowed_packets += 1,
            Action::Block => stats.blocked_packets += 1,
        }

        let elapsed = stats.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            stats.packets_per_second = stats.total_packets as f64 / elapsed;
        }
    }

    fn get_stats(&self) -> FirewallStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }

    fn reset(&self) {
        let mut stats = self.stats.lock().unwrap();
        *stats = FirewallStats::new();
    }
}