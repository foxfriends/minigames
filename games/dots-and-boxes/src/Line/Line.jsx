import React from "react";
import { useDotsAndBoxes, useFns, BOARD_SIZE } from "../DotsAndBoxes";
import { line, horizontal, vertical } from "./Line.module.css";
import classes from "../util/classes";

export default function Line({ start, end }) {
  const { pxy } = useFns();
  const { size } = useDotsAndBoxes();
  const grid = BOARD_SIZE / size;
  const [sx, sy] = pxy(start, size);
  const [dx, dy] = pxy(end, size);

  const cs = [line, sx === dx ? horizontal : vertical];

  const style = {
    left: sx * grid - 1,
    top: sy * grid - 1,
    width: Math.floor(grid * 0.8),
    margin: Math.ceil(grid * 0.1),
  };

  return <div class={classes(cs)} style={style} />;
}
