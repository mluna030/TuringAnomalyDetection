use std::time::Instant;

pub trait RateLimiter: Send + Sync {
    fn is_allowed(&mut self, key: &str) -> bool;
    fn current_usage(&mut self, key: &str) -> Option<f64>;
    fn reset(&mut self, key: &str);
    fn cleanup(&mut self, threshold_secs: u64);
}

#[derive(Debug, Clone)]
pub struct TokenBucket {
    tokens: f64,
    capacity: f64,
    rate: f64,
    last_refill: Instant,
}

impl TokenBucket {
    pub fn new(rate: f64, capacity: f64) -> Self {
        Self  {
            tokens: capacity,
            capacity,
            rate,
            last_refill: Instant::now()
        }
    }

    pub fn try_consume(&mut self, amount: f64) -> bool {
        self.refill();

        if self.tokens >= amount {
            self.tokens -= amount;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        let tokens_to_add = elapsed * self.rate;

        self.tokens = (self.tokens + tokens_to_add).min(self.capacity);
        self.last_refill = now;
    }
    pub fn current_tokens(&mut self) -> f64 {
        self.refill();
        self.tokens
    }

    pub fn set_rate(&mut self, new_rate: f64) {
        if self.rate > 0.0 {
            let ratio = new_rate / self.rate;
            self.tokens = (self.tokens * ratio).min(new_rate);
        }
        self.rate = new_rate;
        self.capacity = new_rate;
    }
}