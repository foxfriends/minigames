use crate::game::Game;
use crate::postgres::PgPool;
use futures::{future, pin_mut, StreamExt};
use std::env;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::unbounded_channel;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tungstenite::Message;

mod context;
mod event;
mod peer_map;
mod subscription_map;

use context::{Context, HandlerContext};
use event::{Action, Event};

async fn handle_action(context: &HandlerContext, action: Action) -> anyhow::Result<()> {
    match action {
        Action::Subscribe(game_id) => {
            context.subscribe(game_id).await;
        }
        Action::Unsubscribe(game_id) => {
            context.unsubscribe(game_id).await;
        }
        Action::Get(game_id) => {
            let mut conn = context.conn().await?;
            let game = Game::load(game_id, &mut conn).await?;
            context.respond_state(game.state).await;
        }
        Action::Set(game_id, new_state) => {
            let mut conn = context.conn().await?;
            Game::update(game_id, new_state.clone(), &mut conn).await?;
            context.broadcast_state(game_id, new_state).await;
        }
    }

    Ok(())
}

async fn handle_connection(
    context: Context,
    stream: TcpStream,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    let stream = tokio_tungstenite::accept_async(stream).await?;

    let (send, recv) = unbounded_channel();
    let (to_socket, from_socket) = stream.split();

    context.peer_map().add(addr, send).await;

    let receiver = from_socket
        .filter_map(|event| async move {
            let msg = event.ok()?;
            let text = msg.into_text().ok()?;
            serde_json::from_str(&text).ok()
        })
        .for_each({
            // Not that it matters... it's just a few Arc anyway.
            let context = context.clone();
            move |Event { id, payload }: Event<Action>| {
                let context = context.to_handler_context(addr, id);
                async move {
                    if let Err(error) = handle_action(&context, payload).await {
                        // TODO: real errors?
                        context.respond_error(error.to_string()).await;
                    }
                }
            }
        });

    let sender = UnboundedReceiverStream::new(recv)
        .map(|event| {
            let json = serde_json::to_string(&event).unwrap();
            Ok(Message::text(json))
        })
        .forward(to_socket);

    pin_mut!(receiver, sender);
    future::select(receiver, sender).await;
    context.peer_map().remove(addr).await;
    Ok(())
}

pub async fn server(pg_pool: PgPool) -> anyhow::Result<()> {
    let ws_port = env::var("WEBSOCKET_PORT")?.parse()?;
    let listener = TcpListener::bind(("0.0.0.0", ws_port)).await?;
    let context = Context::new(pg_pool);

    while let Ok((stream, addr)) = listener.accept().await {
        println!("WebSocket connection from: {}", addr);
        tokio::spawn(handle_connection(context.clone(), stream, addr));
    }

    Ok(())
}
