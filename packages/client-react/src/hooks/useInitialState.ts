import { useEffect } from "react";
import { useGameState } from "../GameStateProvider";
import { useGameInfo } from "../GameInfoProvider";

export default function useInitialState<State>(
  makeState: () => State | Promise<State>,
) {
  const [gameState, setGameState] = useGameState();
  const gameInfo = useGameInfo();

  useEffect(() => {
    async function effect() {
      if (
        gameState === null &&
        !gameInfo.loading &&
        gameInfo.players.find(({ id }) => id === gameInfo.me)?.isChallenger
      ) {
        setGameState(await makeState());
      }
    }
    void effect();
  }, [gameState, gameInfo]);
}
