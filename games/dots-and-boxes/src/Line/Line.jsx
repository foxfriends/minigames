import React from "react";
import { useDotsAndBoxes, BOARD_SIZE } from "../DotsAndBoxes";
import { line, horizontal, vertical } from "./Line.module.css";
import classes from "../util/classes";

export default function Line({ start, end }) {
  const { size } = useDotsAndBoxes();
  const grid = BOARD_SIZE / size;
  const [sx, sy] = [start % (size + 1), Math.floor(start / (size + 1))];
  const [dx, dy] = [end % (size + 1), Math.floor(end / (size + 1))];

  const cs = [line, sx === dx ? horizontal : vertical];

  const style = {
    left: sx * grid - 1,
    top: sy * grid - 1,
    width: Math.floor(grid * 0.8),
    margin: Math.ceil(grid * 0.1),
  };

  return <div class={classes(cs)} style={style} />;
}
