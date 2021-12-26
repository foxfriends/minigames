import React from "react";
import { layout, board, prompt, player } from "./Layout.module.css";
import { slotted } from "../Slot";

export default slotted(function Layout({ slots, children }) {
  return (
    <div className={layout}>
      <div className={prompt}>{slots.prompt}</div>
      <div className={board}>{children}</div>
      <div className={player}>{slots.player}</div>
    </div>
  );
});
