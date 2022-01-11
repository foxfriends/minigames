import { useState, useEffect } from "react";

export type Message = string;

export default function useWebSocket(url: string): WebSocket | null {
  const [socket, setSocket] = useState<WebSocket | null>(null);

  useEffect(() => {
    let ws: WebSocket;

    connect();

    return () => {
      ws.removeEventListener("open", onOpen);
      ws.removeEventListener("close", onClose);
      ws.close();
      setSocket(null);
    };

    function connect() {
      console.log("Connecting WebSocket");
      ws = new WebSocket(url);
      ws.addEventListener("open", onOpen);
      ws.addEventListener("close", onClose);
      setSocket(ws);
    }

    function onOpen() {
      console.log("Socket connected");
    }

    function onClose() {
      console.warn("Socket connection lost... attempting to reconnect");
      connect();
    }
  }, []);

  return socket;
}
