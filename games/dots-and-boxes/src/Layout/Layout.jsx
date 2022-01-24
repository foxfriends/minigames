import React from "react";
import { layout, board, prompt, message } from "./Layout.module.css";
import { slotted } from "../Slot";

export default slotted(function Layout({ slots, children }) {
  return (
    <div className={layout}>
      <div className={prompt}>{slots.prompt}</div>
      <div className={board}>{children}</div>
      <div className={message}>{slots.message}</div>
    </div>
  );
});
