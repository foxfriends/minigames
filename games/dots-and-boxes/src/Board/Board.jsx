import React from "react";
import { useDotsAndBoxes } from "../DotsAndBoxes";
import Line from "../Line";
import Dot from "../Dot";
import Box from "../Box";
import range from "../util/range";
import { board, layer } from "./Board.module.css";

export default function Board() {
  const { gameState, size } = useDotsAndBoxes();

  const dots = range((size + 1) ** 2).map((i) => <Dot index={i} key={i} />);

  const lines = gameState.lines.map(([start, end], i) => (
    <Line start={start} end={end} key={i} />
  ));

  const boxes = gameState.boxes.map((scorer, i) => (
    <Box scorer={scorer} index={i} key={i} />
  ));

  return (
    <>
      <div className={board}>
        <div class={layer}>{dots}</div>
        <div class={layer}>{lines}</div>
        <div class={layer}>{boxes}</div>
      </div>
    </>
  );
}
