import * as React from "react";
import { useGameState } from "../GameStateProvider";
import Cell from "../Cell";
import classes from "../util/classes";
import { board } from "./Board.module.css";

export default function Board() {
  const state = useGameState();
  const cells = state.cells.map((cell, i) => <Cell key={i} {...cell} />);

  return <div className={board}>{cells}</div>;
}
