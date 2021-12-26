import React, { useEffect } from "react";
import GameMetaDataProvider from "../GameMetaDataProvider";
import GameInfoProvider from "../GameInfoProvider";
import GameStateProvider from "../GameStateProvider";

export default function Minigame({
  // Game configuration, provided by user
  name,
  // Base configuration, probably can be defaulted one day when we have an official URL
  socketUrl,
  apiUrl,
  // React
  children,
}) {
  if (process.env.NODE_ENV === "development") {
    useEffect(
      () =>
        console.log(
          `%c${name}%c is running in debug mode. Be sure not to deploy development mode to a real server!`,
          "font-weight: bold",
          "font-weight: default",
        ),
      [],
    );
    // TODO: expose some debug stuff?
  }

  return (
    <GameMetaDataProvider name={name} socketUrl={socketUrl} apiUrl={apiUrl}>
      <GameInfoProvider>
        <GameStateProvider>{children}</GameStateProvider>
      </GameInfoProvider>
    </GameMetaDataProvider>
  );
}
