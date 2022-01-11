import React, { createContext, useContext } from "react";
import type { GameId } from "../types";

const GameMetaDataContext = createContext<GameMetaData>({} as GameMetaData);

export type GameMetaData = {
  gameId: GameId;
  token: string;
  name: string;
  socketUrl: string;
  apiUrl: string;
};

export function useGameMetaData(): GameMetaData {
  return useContext(GameMetaDataContext);
}

export type Props = {
  name: string;
  socketUrl: string;
  apiUrl: string;
  children: React.ReactNode;
};

export default function GameMetaDataProvider({
  name,
  socketUrl,
  apiUrl,
  children,
}: Props) {
  const params = new URLSearchParams(window.location.search);
  const gameId = params.get("game_id") ?? "Unknown";
  const token = params.get("token") ?? "Unauthorized";
  const gameMetaData = { gameId, token, name, socketUrl, apiUrl };

  return (
    <GameMetaDataContext.Provider value={gameMetaData}>
      {children}
    </GameMetaDataContext.Provider>
  );
}
