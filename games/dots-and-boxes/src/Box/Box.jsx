import React from "react";
import { useDotsAndBoxes, useFns, BOARD_SIZE } from "../DotsAndBoxes";
import Avatar from "../Avatar";
import { box } from "./Box.module.css";
import $if from "../util/$if";

export default function Box({ index, scorer }) {
  const { size } = useDotsAndBoxes();
  const { fxy } = useFns();
  const grid = BOARD_SIZE / size;
  const [x, y] = fxy(index);
  const style = {
    left: x * grid,
    top: y * grid,
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
