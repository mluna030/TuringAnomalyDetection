use std::collections::HashMap;
use std::time::Instant;
use super::token_bucket::{TokenBucket, RateLimiter};

pub struct PerKeyRateLimiter {
    buckets: Hashmap<String, TokenBucket>,
    default_rate: f64,
    default_capacity: f64,
}

impl PerKeyRateLimiter {
    pub fn new(rate: f64, capacity: f64) -> Self {
        Self {
            buckets: HashMap::new(),
            default_rate: rate,
            default_capacity: capacity,
        }
    }

    fn get_or_create_bucket(&mut self, key: &str) -> &mut TokenBucket {
        self.buckets.entry(key.to_string()).or_insert_with(|| {
            TokenBucket::new(self.default_rate, self.default_capacity)
        })
    }
}

impl RateLimiter for PerKeyRateLimiter {
    fn is_allowed(&mut self, key: &str) -> bool{
        let bucket = self.get_or_create_bucket(key);
        bucket.try_consume(1.0)
    } 
    fn current_usage(&mut self, key: &str) -> Option<f64>{
        self.buckets.get_mut(key).map(|b| b.current_tokens())
    }
    fn reset(&mut self, key: &str){
        self.buckets.remove(key);
    }
    fn cleanup(&mut self, threshold_secs: u64){
        let now = Instant::now();
        self.buckets.retain(|_, bucket| {
            now.duration_since(bucket.last_refill).as_secs() < threshold_secs
        })
    }
}