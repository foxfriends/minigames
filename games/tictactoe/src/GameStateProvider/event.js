const createEventId = () => {
  let i = 0;

  return function createEventId() {
    return `${++i}`;
  };
};

export function subscribe(gameId) {
  return {
    id: createEventId(),
    payload: { Subscribe: gameId },
  };
}

export function unsubscribe(gameId) {
  return {
    id: createEventId(),
    payload: { Unsubscribe: gameId },
  };
}

export function get(gameId) {
  return {
    id: createEventId(),
    payload: { Get: gameId },
  };
}

export function set(gameId, gameState) {
  return {
    id: createEventId(),
    payload: { Set: [gameId, gameState] },
  };
}
