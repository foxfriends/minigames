import React from "react";
import { useTicTacToe, mark } from "../TicTacToe";
import Layout from "../Layout";
import Board from "../Board";
import Slot from "../Slot";

export default function App() {
  const { gameState, mine, winner } = useTicTacToe();

  function player(symbol) {
    return mark(symbol) ?? "spectating";
  }

  let prompt = "";
  if (winner) {
    prompt = `${mark(winner)} wins`;
  } else if (gameState) {
    prompt = `${mark(gameState.turn)}'s turn`;
  }

  let playerLabel = "";
  if (gameState) {
    playerLabel = `You are ${player(mine)}`;
  }

  return (
    <Layout>
      <Slot name="prompt">{prompt}</Slot>
      <Board />
      <Slot name="player">{playerLabel}</Slot>
    </Layout>
  );
}
