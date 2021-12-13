import * as React from "react";
import { cell } from "./LoadingCell.module.css";

export default function LoadingCell({ index }) {
  const xoffset = 2 - (index % 3);
  const yoffset = Math.floor(index / 3);
  const offset = yoffset - xoffset + 5;
  return (
    <div className={cell} style={{ animationDelay: `${offset * 100}ms` }} />
  );
}
