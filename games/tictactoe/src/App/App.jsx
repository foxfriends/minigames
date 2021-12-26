import React from "react";
import { useTicTacToe, mark } from "../TicTacToe";
import Layout from "../Layout";
import Board from "../Board";
import Slot from "../Slot";

export default function App() {
  const { gameState, mine } = useTicTacToe();

  function player(symbol) {
    return mark(symbol) ?? 'spectating';
  }

  let prompt = "";
  if (gameState) {
    prompt = `${mark(gameState.turn)}'s turn`;
  }

  return (
    <Layout>
      <Slot name="prompt">{prompt}</Slot>
      <Board />
      <Slot name="player">You are {player(mine)}</Slot>
    </Layout>
  );
}
