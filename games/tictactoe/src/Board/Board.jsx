import * as React from "react";
import { useGameState } from "../GameStateProvider";
import Cell from "../Cell";
import LoadingCell from "../LoadingCell";
import classes from "../util/classes";
import range from "../util/range";
import { board } from "./Board.module.css";

export default function Board() {
  const [gameState, setGameState] = useGameState();
  const cells =
    gameState?.cells?.map((cell, i) => <Cell key={i} {...cell} />) ??
    range(0, 9).map((i) => <LoadingCell key={i} index={i} />);
  return <div className={board}>{cells}</div>;
}
