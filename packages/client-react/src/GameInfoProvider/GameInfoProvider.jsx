import React, { createContext, useContext, useState, useEffect } from "react";
import { useGameMetaData } from "../GameMetaDataProvider";
import validate from "../common/jwt";

const GameInfoContext = createContext();

export function useGameInfo() {
  return useContext(GameInfoContext);
}

export default function GameInfoProvider({ children }) {
  const { token, name, gameId, apiUrl } = useGameMetaData();
  const [gameInfo, setGameInfo] = useState({ loading: true });

  useEffect(async () => {
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
  }, []);

  return (
    <GameInfoContext.Provider value={gameInfo}>
      {children}
    </GameInfoContext.Provider>
  );
}
