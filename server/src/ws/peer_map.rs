use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::Mutex;
use tungstenite::Message;

pub type Responder = UnboundedSender<Message>;

#[derive(Clone, Default)]
pub struct PeerMap(Arc<Mutex<HashMap<SocketAddr, Responder>>>);

impl PeerMap {
    pub async fn add(&self, addr: SocketAddr, responder: Responder) {
        let mut map = self.0.lock().await;
        map.insert(addr, responder);
    }

    pub async fn send_to(&self, addr: SocketAddr, event: Message) {
        let map = self.0.lock().await;
        if let Some(responder) = map.get(&addr) {
            responder.send(event).ok();
        }
    }

    pub async fn remove(&self, addr: SocketAddr) {
        let mut map = self.0.lock().await;
        map.remove(&addr);
    }

    pub async fn keys(&self) -> HashSet<SocketAddr> {
        self.0.lock().await.keys().copied().collect()
    }
}
