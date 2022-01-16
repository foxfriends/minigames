import React from "react";
import { useGame } from "../DotsAndBoxes";
import Layout from "../Layout";
import Board from "../Board";
import Slot from "../Slot";

export default function App() {
  const { gameState, mine, winner } = useDotsAndBoxes();

  function player(symbol) {
    return mark(symbol) ?? "spectating";
  }

  let prompt = "";
  if (winner) {
    prompt = `${mark(winner)} wins`;
  } else if (gameState?.cells.every((cell) => cell.value)) {
    prompt = "It's a draw";
  } else if (gameState) {
    prompt = `${mark(gameState.turn)}'s turn`;
  }

  return (
    <Layout>
      <Slot name="prompt">{prompt}</Slot>
      <Board />
    </Layout>
  );
}
