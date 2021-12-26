import { useEffect } from "react";
import { useGameState } from "../GameStateProvider";
import { useGameInfo } from "../GameInfoProvider";

export default function useInitialState(makeState) {
  const [gameState, setGameState] = useGameState();
  const { loading, players, me } = useGameInfo();

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
