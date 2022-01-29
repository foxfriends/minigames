import React, { createContext, useContext, useState, useEffect } from "react";
import { useGameMetaData } from "../GameMetaDataProvider";
import validate from "../common/jwt";
import type { UserId } from "../types";

export type Props = {
  children: React.ReactNode;
};

export type Player = {
  id: UserId;
  isChallenger: boolean;
  username: string;
  discriminator: string;
  avatar: string;
};

type LoadingGameInfo = {
  loading: true;
};

type LoadedGameInfo = {
  loading: false;
  me: UserId;
  players: Player[];
  isComplete: boolean;
  winnerId: UserId | null;
};

export type GameInfo = LoadingGameInfo | LoadedGameInfo;

const GameInfoContext = createContext<GameInfo>({ loading: true });

export function useGameInfo(): GameInfo {
  return useContext(GameInfoContext);
}

export default function GameInfoProvider({ children }: Props) {
  const { token, name, gameId, apiUrl } = useGameMetaData();
  const [gameInfo, setGameInfo] = useState<GameInfo>({ loading: true });

  useEffect(() => {
    async function effect() {
      const { sub: userId } = await validate(token, {
        issuer: apiUrl,
        audience: name,
      });
      const response = await fetch(`${apiUrl}/api/v1/games/${name}/${gameId}`);
      const gameInfo = await response.json();
      setGameInfo({
        loading: false,
        me: userId,
        ...gameInfo,
      });
    }
    void effect();
  }, []);

  return (
    <GameInfoContext.Provider value={gameInfo}>
      {children}
    </GameInfoContext.Provider>
  );
}
