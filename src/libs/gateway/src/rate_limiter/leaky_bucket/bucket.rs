use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct Bucket {
    pub count: u32,
    pub last_updated: DateTime<Utc>,
}

impl Bucket {
    pub fn new() -> Self {
        Bucket {
            count: 0,
            last_updated: Utc::now(),
        }
    }

    pub fn leak(&mut self, leak_rate: f64, capacity: u32) -> bool {
        let now = Utc::now();
        let time_elapsed = (now - self.last_updated).num_milliseconds() as f64 / 1000.0;
        let leaked = (time_elapsed * leak_rate).floor() as u32;
        self.count = self.count.saturating_sub(leaked).min(capacity);
        self.last_updated = now;
        self.count < capacity
    }

    pub fn add(&mut self, tokens: u32, capacity: u32) -> bool {
        if self.count + tokens <= capacity {
            self.count += tokens;
            false
        } else {
            true
        }
    }
}
