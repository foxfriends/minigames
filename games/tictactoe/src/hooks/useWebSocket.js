import { useState, useEffect } from "react";

export default function useWebSocket(url) {
  const [socket, setSocket] = useState(null);

  useEffect(() => {
    let ws;

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
