use super::GameName;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use url::Url;

#[derive(Default)]
pub struct GameRegistry(Arc<Mutex<HashMap<GameName, Url>>>);

impl GameRegistry {
    pub async fn register(&self, game: GameName, url: Url) {
        let mut map = self.0.lock().await;
        map.insert(game, url);
    }

    pub async fn unregister(&self, game: &GameName) {
        let mut map = self.0.lock().await;
        map.remove(game);
    }

    pub async fn locate(&self, game: &GameName) -> Option<Url> {
        self.0.lock().await.get(game).cloned()
    }

    pub async fn list_all(&self) -> Vec<GameName> {
        self.0.lock().await.keys().cloned().collect()
    }
}
