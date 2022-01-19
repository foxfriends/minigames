import React from "react";
import { useDotsAndBoxes, useFns, BOARD_SIZE } from "../DotsAndBoxes";
import { dot } from "./Dot.module.css";

export default function Dot({ index }) {
  const { size } = useDotsAndBoxes();
  const { pxy } = useFns();
  const grid = BOARD_SIZE / size;
  const [x, y] = pxy(index, size);
  const style = {
    left: x * grid - 1,
    top: y * grid - 1,
  };
  return <div class={dot} style={style} />;
}
