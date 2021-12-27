import { useEffect } from "react";
import { useGameMetaData } from "../GameMetaDataProvider";
import { useGameInfo } from "../GameInfoProvider";
import { useGameState } from "../GameStateProvider";

export default function useWinner(computeWinner) {
  const { apiUrl, token, gameId } = useGameMetaData();
  const { me, players } = useGameInfo();
  const [gameState] = useGameState();

  useEffect(async () => {
    if (!players?.find((player) => player.id === me)) {
      return;
    }
    if (!gameState) {
      return;
    }
    const winnerId = await computeWinner(gameState);
    if (winnerId) {
      const response = await fetch(`${apiUrl}/complete`, {
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
  }, [gameState, players, me]);
}
