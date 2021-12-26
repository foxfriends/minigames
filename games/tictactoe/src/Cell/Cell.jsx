import React, { useCallback } from "react";
import { useTicTacToe, X, O, mark } from "../TicTacToe";
import { cell, available, x, o } from "./Cell.module.css";
import classes from "../util/classes";

export default function Cell({ value, onClick }) {
  const { gameState, mine, myTurn } = useTicTacToe();
  const canSelect = myTurn && !value;
  const selectIfAvailable = useCallback(
    () => canSelect && onClick(),
    [canSelect],
  );

  const mineClass = mine === X ? x : o;
  return (
    <div
      className={classes([cell, mineClass, canSelect && available])}
      onClick={selectIfAvailable}
    >
      {mark(value)}
    </div>
  );
}
