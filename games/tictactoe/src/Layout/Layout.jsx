import * as React from "react";
import { layout } from "./Layout.module.css";

export default function Layout({ children }) {
  return <div className={layout}>{children}</div>;
}
