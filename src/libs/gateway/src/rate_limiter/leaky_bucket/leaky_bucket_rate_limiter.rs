use crate::rate_limiter::leaky_bucket::bucket::Bucket;
use std::{collections::HashMap, net::IpAddr, sync::Arc};
use tokio::sync::{Mutex, RwLock};

#[derive(Clone, Debug)]
pub struct LeakyBucketRateLimiter {
    pub buckets: Arc<RwLock<HashMap<IpAddr, Mutex<Bucket>>>>,
    pub capacity: u32,
    pub leak_rate: f64, // tokens per second
}

impl LeakyBucketRateLimiter {
    pub fn new(capacity: u32, leak_rate: f64) -> Self {
        LeakyBucketRateLimiter {
            buckets: Arc::new(RwLock::new(HashMap::new())),
            capacity,
            leak_rate,
        }
    }

    pub async fn check_rate_limit(&self, ip: IpAddr) -> bool {
        dbg!(self);

        {
            let buckets = self.buckets.read().await;
            if let Some(bucket) = buckets.get(&ip) {
                let mut bucket = bucket.lock().await;
                if bucket.leak(self.leak_rate, self.capacity) {
                    return bucket.add(1, self.capacity);
                } else {
                    return true;
                }
            }
        }

        let mut buckets = self.buckets.write().await;
        buckets.insert(ip, Mutex::new(Bucket::new()));

        // dbg!(self);
        false
    }
}
