import * as React from "react";
import Layout from "../Layout";
import Board from "../Board";
import GameStateProvider from "../GameStateProvider";

export default function App() {
  return (
    <GameStateProvider>
      <Layout>
        <Board />
      </Layout>
    </GameStateProvider>
  );
}
