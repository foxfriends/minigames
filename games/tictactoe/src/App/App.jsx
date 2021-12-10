import * as React from "react";
import Layout from "../Layout";
import Board from "../Board";
import GameStateProvider from "../GameStateProvider";

export default function App({ gameId }) {
  return (
    <GameStateProvider gameId={gameId}>
      <Layout>
        <Board />
      </Layout>
    </GameStateProvider>
  );
}
