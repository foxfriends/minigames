use super::peer_map::PeerMap;
use crate::game::GameId;
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Default)]
pub struct SubscriptionMap(Arc<Mutex<HashMap<GameId, HashSet<SocketAddr>>>>);

impl SubscriptionMap {
    pub async fn subscribe(&self, game_id: GameId, addr: SocketAddr) {
        let mut map = self.0.lock().await;
        map.entry(game_id).or_default().insert(addr);
    }

    pub async fn unsubscribe(&self, game_id: GameId, addr: SocketAddr) {
        let mut map = self.0.lock().await;
        if let Some(set) = map.get_mut(&game_id) {
            set.remove(&addr);
            if set.is_empty() {
                map.remove(&game_id);
            }
        }
    }

    pub async fn get(&self, game_id: GameId, peer_map: &PeerMap) -> HashSet<SocketAddr> {
        let mut map = self.0.lock().await;
        let existing = peer_map.keys().await;
        if let Some(set) = map.get_mut(&game_id) {
            set.retain(|addr| existing.contains(addr));
            set.clone()
        } else {
            HashSet::default()
        }
    }
}
