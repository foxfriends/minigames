import React, { createContext, useContext } from "react";

const GameMetaDataContext = createContext();

export function useGameMetaData() {
  return useContext(GameMetaDataContext);
}

export default function GameMetaDataProvider({
  name,
  socketUrl,
  apiUrl,
  children,
}) {
  const params = new URLSearchParams(window.location.search);
  const gameId = params.get("game_id");
  const token = params.get("token");
  const gameMetaData = { gameId, token, name, socketUrl, apiUrl };

  return (
    <GameMetaDataContext.Provider value={gameMetaData}>
      {children}
    </GameMetaDataContext.Provider>
  );
}
