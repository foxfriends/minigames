import React from "react";
import { useDotsAndBoxes, BOARD_SIZE } from "../DotsAndBoxes";
import { dot } from "./Dot.module.css";

export default function Dot({ index }) {
  const { size } = useDotsAndBoxes();
  const grid = BOARD_SIZE / size;
  const style = {
    left: (index % (size + 1)) * grid - 1,
    top: Math.floor(index / (size + 1)) * grid - 1,
  };
  return <div class={dot} style={style} />;
}
