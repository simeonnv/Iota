use std::collections::HashMap;

use tokio::sync::watch;
use uuid::Uuid;

mod nat;
use nat::Nat;

use crate::nat_subscriber::nat::NatBody;
pub struct NatSubsciber {
    pub nats: HashMap<Uuid, Nat>,
}

impl NatSubsciber {
    pub fn new() -> Self {
        Self {
            nats: HashMap::new(),
        }
    }

    pub fn subscribe_or_init(&mut self, account_uuid: &Uuid) -> watch::Receiver<Option<NatBody>> {
        let nat = self
            .nats
            .entry(*account_uuid) // now we store by value
            .or_insert_with(|| Nat::new());

        nat.subscribe()
    }

    pub fn get_sender_or_init(&mut self, account_uuid: &Uuid) -> watch::Sender<Option<NatBody>> {
        let nat = self
            .nats
            .entry(*account_uuid) // now we store by value
            .or_insert_with(|| Nat::new());

        nat.get_sender()
    }
}
