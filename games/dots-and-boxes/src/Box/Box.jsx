import React from "react";
import { useDotsAndBoxes, BOARD_SIZE } from "../DotsAndBoxes";
import { box } from "./Box.module.css";
import $if from "../util/$if";

export default function Box({ index, scorer }) {
  const { size } = useDotsAndBoxes();
  const grid = BOARD_SIZE / size;
  const style = {
    left: (index % size) * grid,
    top: Math.floor(index / size) * grid,
    width: grid,
    height: grid,
  };
  return (
    <div class={box} style={style}>
      {$if(scorer, () => (
        <Avatar id={scorer} />
      ))}
    </div>
  );
}
