use chrono::{NaiveDateTime, Utc};
use tokio::sync::watch;

pub struct NatBody {
    pub ip: String,
}

pub struct Nat {
    pub sen: watch::Sender<Option<NatBody>>,
    pub last_seen: NaiveDateTime,
}

impl Nat {
    pub fn new() -> Self {
        let (sen, _) = watch::channel(None);
        let now = Utc::now().naive_utc();

        Nat {
            sen,
            last_seen: now,
        }
    }
    pub fn subscribe(&self) -> watch::Receiver<Option<NatBody>> {
        self.sen.subscribe()
    }
    pub fn get_sender(&self) -> watch::Sender<Option<NatBody>> {
        self.sen.clone()
    }
}
