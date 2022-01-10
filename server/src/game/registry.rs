use super::{GameName, GameServer};
use sqlx::postgres::Postgres;
use sqlx::Executor;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct GameServerState {
    pub enabled: bool,
    pub available: bool,
    pub public_url: String,
}

pub struct GameRegistry(Arc<Mutex<HashMap<GameName, GameServerState>>>);

impl GameRegistry {
    pub async fn initialize<Conn>(conn: Conn) -> anyhow::Result<Self>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        let mut map = HashMap::default();
        for server in GameServer::list_all(conn).await? {
            map.insert(
                server.name().clone(),
                GameServerState {
                    enabled: server.enabled,
                    available: true,
                    public_url: server.public_url.clone(),
                },
            );
        }
        Ok(Self(Arc::new(Mutex::new(map))))
    }

    pub async fn register(&self, server: &GameServer) {
        let mut map = self.0.lock().await;
        map.insert(
            server.name().clone(),
            GameServerState {
                enabled: server.enabled,
                available: true,
                public_url: server.public_url.clone(),
            },
        );
    }

    pub async fn unregister(&self, game: &GameName) {
        let mut map = self.0.lock().await;
        map.remove(game);
    }

    pub async fn set_available(&self, game: &GameName, available: bool) {
        let mut map = self.0.lock().await;
        if let Some(state) = map.get_mut(game) {
            state.available = available;
        }
    }

    pub async fn locate(&self, game: &GameName) -> Option<String> {
        Some(self.0.lock().await.get(game)?.public_url.to_owned())
    }

    pub async fn list_available(&self) -> Vec<GameName> {
        self.0
            .lock()
            .await
            .iter()
            .filter(|(_, state)| state.enabled && state.available)
            .map(|(key, _)| key)
            .cloned()
            .collect()
    }
}
