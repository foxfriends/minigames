import React, { createContext, useContext, useCallback, useState } from "react";
import * as event from "../common/event";
import useEvent from "../hooks/useEvent";
import useWebSocket from "../hooks/useWebSocket";
import { useGameMetaData } from "../GameMetaDataProvider";

const GameStateContext = createContext();

export function useGameState() {
  return useContext(GameStateContext);
}

export default function GameStateProvider({ children }) {
  const { gameId, socketUrl, token } = useGameMetaData();
  const [state, setState] = useState();
  const socket = useWebSocket(`${socketUrl}?token=${token}`);

  const onMessage = useCallback(
    (message) => {
      const data = JSON.parse(message.data);
      if ("Update" in data.payload) {
        setState(data.payload.Update);
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

  function setGameState(newState) {
    socket.send(event.set(gameId, newState));
  }

  return (
    <GameStateContext.Provider value={[state, setGameState]}>
      {children}
    </GameStateContext.Provider>
  );
}
