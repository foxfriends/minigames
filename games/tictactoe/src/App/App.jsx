import React, { useEffect } from "react";
import { useMinigame } from "@minigames/react";
import Layout from "../Layout";
import Board from "../Board";

export default function App() {
  const { gameState, setGameState, me, players } = useMinigame();

  // TODO: this is not a particularly elegant way to set up the initial state...
  useEffect(() => {
    if (
      gameState === null &&
      players?.find(({ id }) => id === me)?.isChallenger
    ) {
      const x = Math.floor(Math.random() * 2);
      setGameState({
        x: players[x].id,
        o: players[1 - x].id,
        cells: [
          { value: null },
          { value: null },
          { value: null },
          { value: null },
          { value: null },
          { value: null },
          { value: null },
          { value: null },
          { value: null },
        ],
      });
    }
  }, [gameState, me, players]);

  return (
    <Layout>
      <Board />
    </Layout>
  );
}
