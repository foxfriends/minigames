import { useState, useEffect } from "react";
import { useGameMetaData } from "../GameMetaDataProvider";
import { useGameInfo } from "../GameInfoProvider";
import { useGameState } from "../GameStateProvider";
import type { UserId } from "../types";

export default function useWinner<State>(
  computeWinner: (state: State) => UserId | null | undefined,
): UserId | null | undefined {
  const { apiUrl, token, gameId } = useGameMetaData();
  const gameInfo = useGameInfo();
  const [gameState] = useGameState<State>();

  const [winner, setWinner] = useState<UserId | null | undefined>(undefined);

  useEffect(() => {
    async function effect() {
      if (gameInfo.loading) {
        return;
      }
      if (!gameInfo.players?.find((player) => player.id === gameInfo.me)) {
        return;
      }
      if (!gameState) {
        return;
      }
      const winnerId = await computeWinner(gameState);
      if (winnerId !== undefined) {
        setWinner(winnerId);
        const response = await fetch(`${apiUrl}/api/v1/complete`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${token}`,
          },
          body: JSON.stringify({ gameId, winnerId }),
        });
        if (response.status !== 200) {
          console.error(await response.json());
          return;
        }
      }
    }
    void effect();
  }, [gameState, gameInfo]);

  return winner;
}
