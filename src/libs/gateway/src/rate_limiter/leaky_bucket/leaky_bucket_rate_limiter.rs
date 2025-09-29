use crate::rate_limiter::leaky_bucket::bucket::Bucket;
use dashmap::DashMap;
use std::net::IpAddr;

#[derive(Clone, Debug)]
pub struct LeakyBucketRateLimiter {
    pub buckets: DashMap<IpAddr, Bucket>,
    pub capacity: u32,
    pub leak_rate: f64, // tokens per second
}

impl LeakyBucketRateLimiter {
    pub fn new(capacity: u32, leak_rate: f64) -> Self {
        LeakyBucketRateLimiter {
            buckets: DashMap::new(),
            capacity,
            leak_rate,
        }
    }

    pub async fn check_rate_limit(&self, ip: IpAddr) -> bool {
        if let Some(mut bucket) = self.buckets.get_mut(&ip) {
            if bucket.leak(self.leak_rate, self.capacity) {
                return bucket.add(1, self.capacity);
            } else {
                return true;
            }
        }

        self.buckets.insert(ip, Bucket::new());
        false
    }
}
