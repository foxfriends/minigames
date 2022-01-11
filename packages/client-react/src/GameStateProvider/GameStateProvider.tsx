import React, { createContext, useContext, useCallback, useState } from "react";
import * as event from "../common/event";
import useEvent from "../hooks/useEvent";
import useWebSocket from "../hooks/useWebSocket";
import { useGameMetaData } from "../GameMetaDataProvider";

const GameStateContext = createContext([
  undefined,
  (state: any | null) => {},
] as [any, (state: any | null) => void]);

export function useGameState<State>(): [State, (state: State) => void] {
  return useContext(GameStateContext);
}

export type Props = {
  children: React.ReactNode;
};

export default function GameStateProvider<State>({ children }: Props) {
  const { gameId, socketUrl, token } = useGameMetaData();
  const [state, setState] = useState<State | null | undefined>();
  const socket = useWebSocket(`${socketUrl}?token=${token}`);

  const onMessage = useCallback(
    (message: MessageEvent) => {
      const data = JSON.parse(message.data);
      if ("Update" in data.payload) {
        setState(data.payload.Update);
      } else {
        console.error("Unexpected WebSocket event:", data);
      }
    },
    [setState],
  );

  useEvent(socket, "message", onMessage as EventListener);
  useEvent(socket, "open", () => {
    socket!.send(event.subscribe(gameId));
    socket!.send(event.get(gameId));
  });

  function setGameState(newState: State | null) {
    socket!.send(event.set(gameId, newState));
  }

  return (
    <GameStateContext.Provider value={[state, setGameState]}>
      {children}
    </GameStateContext.Provider>
  );
}
