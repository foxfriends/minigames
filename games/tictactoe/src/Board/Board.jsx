import React from "react";
import { useTicTacToe } from "../TicTacToe";
import Cell from "../Cell";
import LoadingCell from "../LoadingCell";
import range from "../util/range";
import { board, line } from "./Board.module.css";

export default function Board() {
  const { gameState, select, winningCells } = useTicTacToe();

  const cells =
    gameState?.cells?.map((cell, i) => (
      <Cell key={i} {...cell} onClick={() => select(i)} />
    )) ?? range(0, 9).map((i) => <LoadingCell key={i} index={i} />);

  let winningLine = null;
  if (winningCells) {
    const top = Math.floor(winningCells[0] / 3);
    const left = winningCells[0] % 3;
    const bottom = Math.floor(winningCells[2] / 3);
    const right = winningCells[2] % 3;

    winningLine = (
      <svg className={line} viewBox="0 0 6 6">
        <line x1={left * 2 + 1} y1={top * 2 + 1} x2={right * 2 + 1} y2={bottom * 2 + 1} />
      </svg>
    );
  }

  return (
    <>
      <div className={board}>{cells}</div>
      {winningLine}
    </>
  );
}
