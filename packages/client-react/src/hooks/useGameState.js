import { useCallback, useState } from "react";
import * as event from "../common/event";
import useEvent from "./useEvent";
import useWebSocket from "./useWebSocket";

export default function useGameState(gameId, { socketUrl, token }) {
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

  return [state, setGameState];
}
