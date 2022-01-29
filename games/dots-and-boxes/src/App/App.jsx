import React, { useEffect } from "react";
import { useGameInfo } from "@foxfriends/minigames-client-react";
import { useDotsAndBoxes } from "../DotsAndBoxes";
import Layout from "../Layout";
import Board from "../Board";
import Slot from "../Slot";

export default function App() {
  const { loading, players } = useGameInfo();
  const { gameState, me, mine, winner } = useDotsAndBoxes();

  useEffect(() => {
    window.GAME_STATE = gameState;
  }, [gameState]);

  if (loading || gameState === null) {
    return <Layout>Loading...</Layout>;
  }

  function name(player) {
    return players.find(({ id }) => id === player)?.username ?? "spectating";
  }

  const message = `You are ${name(me)}`;

  let prompt = "";
  if (winner) {
    prompt = `${name(winner)} wins`;
  } else if (winner === null) {
    prompt = "It's a draw";
  } else if (gameState) {
    prompt = `${name(gameState.turn)}'s turn`;
  }

  return (
    <Layout>
      <Slot name="prompt">{prompt}</Slot>
      <Board />
      <Slot name="message">{message}</Slot>
    </Layout>
  );
}
