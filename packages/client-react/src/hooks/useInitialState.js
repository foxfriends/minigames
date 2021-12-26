import { useEffect } from "react";
import { useMinigame } from "../Minigame";

export default function useInitialState(makeState) {
  const { gameState, setGameState, loading, players, me } = useMinigame();

  useEffect(async () => {
    if (
      gameState === null &&
      !loading &&
      players?.find(({ id }) => id === me)?.isChallenger
    ) {
      setGameState(await makeState())
    }
  }, [gameState, loading]);
}
