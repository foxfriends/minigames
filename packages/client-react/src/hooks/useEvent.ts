import { useEffect } from "react";

export default function useEvent(
  eventEmitter: EventTarget | null,
  event: string,
  callback: EventListenerOrEventListenerObject,
) {
  useEffect(() => {
    eventEmitter?.addEventListener(event, callback);
    return () => eventEmitter?.removeEventListener(event, callback);
  }, [eventEmitter, callback]);
}
