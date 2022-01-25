import React from "react";
import { useDotsAndBoxes, BOARD_SIZE } from "../DotsAndBoxes";
import { avatar } from "./Avatar.module.css";

export default function Avatar({ id }) {
  const { size } = useDotsAndBoxes();
  const grid = BOARD_SIZE / size;
  const url = "/"; // TODO: discord API
  return (
    <img
      className={avatar}
      src={url}
      alt={id[0]}
      style={{ width: grid * 0.6, height: grid * 0.6 }}
    />
  );
}
