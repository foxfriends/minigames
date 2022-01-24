import React from "react";
import { useDotsAndBoxes, useFns, line } from "../DotsAndBoxes";
import Line from "../Line";
import Dot from "../Dot";
import Box from "../Box";
import range from "../util/range";
import { board, layer } from "./Board.module.css";

export default function Board() {
  const { gameState, size, isMyTurn, drawLine } = useDotsAndBoxes();
  const { pxy, xyp } = useFns();

  const dotIndices = range(0, (size + 1) ** 2);
  const dots = dotIndices.map((i) => <Dot index={i} key={i} />);

  const lineIndices = dotIndices.flatMap((start) => {
    const [x, y] = pxy(start);
    return [
      [x + 1, y],
      [x, y + 1],
    ]
      .filter((xy) => !xy.includes(size + 1))
      .map(([x, y]) => xyp(x, y))
      .map((end) => [start, end]);
  });

  const lines = lineIndices.map((l, i) => (
    <Line
      line={l}
      drawn={gameState.lines.some((l2) => line.eq(l, l2))}
      onClick={() => isMyTurn && drawLine(l)}
      key={i}
    />
  ));

  const boxes =
    gameState?.boxes.map((scorer, i) => (
      <Box scorer={scorer} index={i} key={i} />
    )) ?? [];

  return (
    <>
      <div className={board}>
        <div className={layer}>{dots}</div>
        <div className={layer}>{boxes}</div>
        <div className={layer}>{lines}</div>
      </div>
    </>
  );
}
