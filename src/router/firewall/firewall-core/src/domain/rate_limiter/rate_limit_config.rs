
#[derive(Debug, Clone)]
pub enum RateLimitKeyType {
    SourceIp,
    DestinationIp,
    Flow,
    Global,    
}

#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub rate: f64,
    pub capacity: f64,
    pub key_type: RateLimitKeyType,
}

impl RateLimitConfig {
    pub fn new(rate: f64, capacity: f64) -> Self {
        Self {
            rate,
            capacity,
            key_type: RateLimitKeyType::SourceIp,
        }
    }

    pub fn per_source_ip(rate: f64, capacity: f64) -> Self {
        Self {
            rate,
            capacity,
            key_type: RateLimitKeyType::SourceIp,
        }
    }

    pub fn per_destination_ip(rate: f64, capacity: f64) -> Self {
        Self {
            rate,
            capacity,
            key_type: RateLimitKeyType::DestinationIp,
        }
    }
}