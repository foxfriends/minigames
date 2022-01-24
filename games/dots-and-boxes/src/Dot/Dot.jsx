import React from "react";
import { useDotsAndBoxes, useFns, BOARD_SIZE } from "../DotsAndBoxes";
import { dot } from "./Dot.module.css";

export default function Dot({ index }) {
  const { gameState, size } = useDotsAndBoxes();
  const { pxy } = useFns();
  const grid = BOARD_SIZE / size;
  const [x, y] = pxy(index);
  const style = {
    left: x * grid,
    top: y * grid,
  };
  return <div className={dot} style={style} />;
}
