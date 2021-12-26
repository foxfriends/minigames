import React from "react";
import { useTicTacToe, mark } from "../TicTacToe";
import Layout from "../Layout";
import Board from "../Board";
import Slot from "../Slot";

export default function App() {
  const { gameState } = useTicTacToe();

  let prompt = "";
  if (gameState) {
    prompt = `${mark(gameState.turn)}'s turn`;
  }

  return (
    <Layout>
      <Slot name="prompt">{prompt}</Slot>

      <Board />
    </Layout>
  );
}
