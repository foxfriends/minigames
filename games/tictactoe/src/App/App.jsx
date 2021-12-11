import * as React from "react";
import Layout from "../Layout";
import Board from "../Board";
import GameStateProvider from "../GameStateProvider";

export default function App({ gameId, userId, token }) {
  return (
    <GameStateProvider gameId={gameId} userId={userId} token={token}>
      <Layout>
        <Board />
      </Layout>
    </GameStateProvider>
  );
}
