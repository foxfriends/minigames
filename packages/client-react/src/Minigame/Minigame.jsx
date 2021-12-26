import React, { createContext, useContext, useState, useEffect } from "react";
import validate from "../common/jwt";
import useGameState from "../hooks/useGameState";

const MinigameContext = createContext();

export function useMinigame() {
  return useContext(MinigameContext);
}

export default function Minigame({
  // Game configuration, provided by user
  name,
  // Base configuration, probably can be defaulted one day when we have an official URL
  socketUrl,
  apiUrl,
  // React
  children,
}) {
  const params = new URLSearchParams(window.location.search);
  const gameId = params.get("game_id");
  const token = params.get("token");

  const [gameState, setGameState] = useGameState(gameId, { socketUrl, token });

  const [gameInfo, setGameInfo] = useState(null);

  useEffect(async () => {
    const { sub: userId } = await validate(token, {
      issuer: apiUrl,
      audience: name,
    });
    const response = await fetch(`${apiUrl}/games/${name}/${gameId}`);
    const gameInfo = await response.json();
    setGameInfo({
      me: userId,
      ...gameInfo,
    });
  }, []);

  const minigame = {
    gameId,
    loading: gameInfo === null,
    ...gameInfo,
    gameState,
    setGameState,
  };

  if (process.env.NODE_ENV === 'development') {
    useEffect(() => console.log(`%c${name}%c is running in debug mode. Global variable "_MinigamesDebug_" describes internal game state. Be sure not to run in development mode on production!`, 'font-weight: bold', 'font-weight: default'), []);
    useEffect(() => window._MinigamesDebug_ = minigame, [minigame]);
  }

  return (
    <MinigameContext.Provider value={minigame}>
      {children}
    </MinigameContext.Provider>
  );
}
