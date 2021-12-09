import { useEffect } from "react";

export default function useEvent(eventEmitter, event, callback) {
  useEffect(() => {
    eventEmitter?.addEventListener(event, callback);
    return () => eventEmitter?.removeEventListener(event, callback);
  }, [eventEmitter, callback]);
}
