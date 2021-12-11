import * as React from "react";
import {
  createContext,
  useContext,
  useState,
  useEffect,
  useCallback,
} from "react";
import useWebSocket from "../hooks/useWebSocket";
import useEvent from "../hooks/useEvent";
import * as event from "./event";

const DEFAULT = {
  cells: [
    { value: null },
    { value: null },
    { value: null },
    { value: null },
    { value: null },
    { value: null },
    { value: null },
    { value: null },
    { value: null },
  ],
};

const GameStateContext = createContext(DEFAULT);

export function useGameState() {
  return useContext(GameStateContext);
}

export default function GameStateProvider({ gameId, token, children }) {
  const [state, setState] = useState(DEFAULT);
  const socket = useWebSocket(`${import.meta.env.VITE_SOCKET_URL}?token=${token}`);

  const onMessage = useCallback(
    (message) => {
      const data = JSON.parse(message.data);
      if ("Update" in data.payload) {
        setState(data.payload.Update ?? DEFAULT);
      } else {
        console.error("Unexpected WebSocket event:", data);
      }
    },
    [setState],
  );

  useEvent(socket, "message", onMessage);
  useEvent(socket, "open", () => {
    socket.send(event.subscribe(gameId));
    socket.send(event.get(gameId));
  });

  const setGameState = (newState) => socket.send(event.set(gameId, newState));

  return (
    <GameStateContext.Provider value={[state, setGameState]}>
      {children}
    </GameStateContext.Provider>
  );
}
