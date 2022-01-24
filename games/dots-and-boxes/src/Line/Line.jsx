import React from "react";
import { useDotsAndBoxes, useFns, BOARD_SIZE } from "../DotsAndBoxes";
import {
  container,
  disabled,
  line,
  horizontal,
  vertical,
  undrawn,
} from "./Line.module.css";
import classes from "../util/classes";

export default function Line({
  line: [start, end],
  drawn: isDrawn = false,
  onClick,
}) {
  const { pxy } = useFns();
  const { size, isMyTurn } = useDotsAndBoxes();
  const grid = BOARD_SIZE / size;
  const [sx, sy] = pxy(start);
  const [dx, dy] = pxy(end);

  const cs = [
    sx === dx ? vertical : horizontal,
    isDrawn ? undefined : undrawn,
    isMyTurn ? undefined : disabled,
  ];

  const left = sx * grid - 1;
  const top = sy * grid - 1;
  const length = Math.floor(grid * 0.8);
  const direction = sx === dx ? "height" : "width";
  const notDirection = sx === dx ? "width" : "height";
  const padding = Math.ceil(grid * 0.1);

  return (
    <div
      className={classes([container, ...cs])}
      style={{ top, left, padding }}
      onClick={onClick}
    >
      <div
        className={classes([line, ...cs])}
        style={{ [direction]: length, [notDirection]: 2 }}
      />
    </div>
  );
}
