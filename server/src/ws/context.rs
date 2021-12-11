use super::event::{Event, EventId, Response};
use super::peer_map::PeerMap;
use super::subscription_map::SubscriptionMap;
use crate::game::{Game, GameId, GameState};
use crate::postgres::PgPool;
use crate::token::Claims;
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, Postgres};
use std::net::SocketAddr;

#[derive(Clone)]
pub struct Context {
    pg_pool: PgPool,
    peer_map: PeerMap,
    subscription_map: SubscriptionMap,
}

impl Context {
    pub fn new(pg_pool: PgPool) -> Self {
        Self {
            pg_pool,
            peer_map: PeerMap::default(),
            subscription_map: SubscriptionMap::default(),
        }
    }

    pub fn peer_map(&self) -> &PeerMap {
        &self.peer_map
    }

    pub fn to_handler_context<'c>(
        &self,
        addr: SocketAddr,
        claims: Option<&'c Claims>,
        event_id: EventId,
    ) -> HandlerContext<'c> {
        HandlerContext {
            addr,
            context: self.clone(),
            event_id,
            claims,
        }
    }
}

pub struct HandlerContext<'c> {
    addr: SocketAddr,
    context: Context,
    event_id: EventId,
    claims: Option<&'c Claims>,
}

impl HandlerContext<'_> {
    async fn send(&self, to_addr: SocketAddr, response: Event<Response>) {
        self.context.peer_map.send_to(to_addr, response).await;
    }

    pub async fn broadcast_state(&self, game_id: GameId, data: GameState) {
        let subscribed = self
            .context
            .subscription_map
            .get(game_id, &self.context.peer_map)
            .await;

        let event = Event {
            id: EventId::default(),
            payload: Response::Update(data),
        };
        for addr in subscribed {
            self.send(addr, event.clone()).await;
        }
    }

    pub async fn respond_state(&self, data: GameState) {
        self.send(
            self.addr,
            Event {
                id: self.event_id.clone(),
                payload: Response::Update(data),
            },
        )
        .await;
    }

    pub async fn respond_error(&self, error: String) {
        self.send(
            self.addr,
            Event {
                id: self.event_id.clone(),
                payload: Response::Error(error),
            },
        )
        .await;
    }

    pub async fn conn(&self) -> anyhow::Result<PoolConnection<Postgres>> {
        Ok(self.context.pg_pool.acquire().await?)
    }

    pub async fn is_participant(
        &self,
        game: &Game,
        conn: &mut PgConnection,
    ) -> anyhow::Result<bool> {
        match self.claims {
            Some(claims) => {
                Ok(claims.is_game(&game.game)
                    && game.is_participant(claims.user_id(), conn).await?)
            }
            None => Ok(false),
        }
    }

    pub async fn subscribe(&self, game_id: GameId) {
        self.context
            .subscription_map
            .subscribe(game_id, self.addr)
            .await;
    }

    pub async fn unsubscribe(&self, game_id: GameId) {
        self.context
            .subscription_map
            .unsubscribe(game_id, self.addr)
            .await;
    }
}
