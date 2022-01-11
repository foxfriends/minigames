import type { GameId } from "../types";
import type { Message } from "../hooks/useWebSocket";

const createEventId = (() => {
  let i = 0;

  return function createEventId() {
    return `${++i}`;
  };
})();

export function subscribe(gameId: GameId): Message {
  return JSON.stringify({
    id: createEventId(),
    payload: { Subscribe: gameId },
  });
}

export function unsubscribe(gameId: GameId): Message {
  return JSON.stringify({
    id: createEventId(),
    payload: { Unsubscribe: gameId },
  });
}

export function get(gameId: String): Message {
  return JSON.stringify({
    id: createEventId(),
    payload: { Get: gameId },
  });
}

export function set<T>(gameId: GameId, gameState: T): Message {
  return JSON.stringify({
    id: createEventId(),
    payload: { Set: [gameId, gameState] },
  });
}
