use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::Arc,
};

use drift_pubsub_client::PubsubClient;
use solana_sdk::pubkey::Pubkey;

use crate::{types::SdkResult, MarketId};

#[derive(Clone, Debug)]
pub struct PubsubPool {
    clients: Vec<Arc<PubsubClient>>,
}

impl PubsubPool {
    pub async fn new(ws_url: &str, size: usize) -> SdkResult<Self> {
        let pool_size = size.max(1);
        let mut clients = Vec::with_capacity(pool_size);
        for _ in 0..pool_size {
            clients.push(Arc::new(PubsubClient::new(ws_url).await?));
        }
        Ok(Self { clients })
    }

    pub fn single(client: Arc<PubsubClient>) -> Self {
        Self { clients: vec![client] }
    }

    pub fn len(&self) -> usize {
        self.clients.len()
    }

    pub fn any(&self) -> Arc<PubsubClient> {
        self.clients[0].clone()
    }

    pub fn pick_by_pubkey(&self, pubkey: &Pubkey) -> Arc<PubsubClient> {
        if self.clients.len() == 1 {
            return self.any();
        }
        let mut hasher = DefaultHasher::new();
        pubkey.to_bytes().hash(&mut hasher);
        let idx = (hasher.finish() as usize) % self.clients.len();
        self.clients[idx].clone()
    }

    pub fn pick_by_market_id(&self, market: &MarketId) -> Arc<PubsubClient> {
        if self.clients.len() == 1 {
            return self.any();
        }
        let idx = (market.index() as usize) % self.clients.len();
        self.clients[idx].clone()
    }
}
