import React from "react";
import { useGameInfo } from "@foxfriends/minigames-client-react";
import { useDotsAndBoxes, BOARD_SIZE } from "../DotsAndBoxes";
import { avatar } from "./Avatar.module.css";

export default function Avatar({ id }) {
  const { loading, players } = useGameInfo();
  const { size } = useDotsAndBoxes();
  const player = players.find((player) => player.id === id);
  if (!player) {
    return null;
  }
  const grid = BOARD_SIZE / size;
  const url = `https://cdn.discordapp.com/avatars/${id}/${player.avatar}.png?size=128`;
  return (
    <img
      className={avatar}
      src={url}
      alt={player.username[0]}
      style={{ width: grid * 0.6, height: grid * 0.6 }}
    />
  );
}
