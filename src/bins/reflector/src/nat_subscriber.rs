use std::collections::HashMap;

use chrono::NaiveDateTime;
use serde::Serialize;
use tokio::sync::watch::{self, Receiver, Sender};
use uuid::Uuid;

#[derive(Debug)]
pub struct NatSubsciber {
    pub nats: HashMap<Uuid, Nat>,
}

#[derive(Debug, Clone)]
pub struct Nat {
    pub sender: Sender<Option<NatBody>>,
    pub receiver: Receiver<Option<NatBody>>,
}

#[derive(Debug, Serialize)]
pub struct NatBody {
    pub ip: String,
    pub last_updated: NaiveDateTime,
}

impl NatSubsciber {
    pub fn new() -> Self {
        Self {
            nats: HashMap::new(),
        }
    }

    pub fn init_nat(&mut self, account_uuid: Uuid) -> Nat {
        let channel = watch::channel(None);
        let nat = Nat {
            sender: channel.0,
            receiver: channel.1,
        };
        self.nats.insert(account_uuid, nat.clone());
        nat
    }

    pub fn subscribe(&self, account_uuid: &Uuid) -> Option<Receiver<Option<NatBody>>> {
        let nat = self.nats.get(account_uuid);
        nat.map(|e| e.receiver.clone())
    }

    pub fn get_sender(&self, account_uuid: &Uuid) -> Option<Sender<Option<NatBody>>> {
        let nat = self.nats.get(account_uuid);
        nat.map(|e| e.sender.clone())
    }
}
