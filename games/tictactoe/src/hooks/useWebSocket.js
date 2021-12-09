import { useState, useEffect } from "react";

export default function useWebSocket(url, callbacks) {
  const [socket, setSocket] = useState(null);

  useEffect(() => {
    const ws = new WebSocket(url);

    function onOpen() {
      console.log('Socket connected');
    }

    function onClose() {
      console.warn('Socket connection lost... attempting to reconnect');
      // TODO: attempt to reconnect...
    }

    ws.addEventListener('open', onOpen);
    ws.addEventListener('close', onClose);

    setSocket(ws);

    return () => {
      ws.close();
      ws.removeEventListener('close', onClose);
      setSocket(null);
    };
  }, []);

  return socket;
}
