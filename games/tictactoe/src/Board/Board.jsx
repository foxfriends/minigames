import React from "react";
import { useTicTacToe } from "../TicTacToe";
import Cell from "../Cell";
import LoadingCell from "../LoadingCell";
import range from "../util/range";
import { board } from "./Board.module.css";

export default function Board() {
  const { gameState, select } = useTicTacToe();

  const cells =
    gameState?.cells?.map((cell, i) => (
      <Cell key={i} {...cell} onClick={() => select(i)} />
    )) ?? range(0, 9).map((i) => <LoadingCell key={i} index={i} />);

  return <div className={board}>{cells}</div>;
}
