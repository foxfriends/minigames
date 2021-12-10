import * as React from "react";
import { render } from "react-dom";
import App from "./App";
import "./index.css";

const gameId = new URLSearchParams(window.location.search).get("game_id");

render(<App gameId={gameId} />, document.querySelector("#app"));
