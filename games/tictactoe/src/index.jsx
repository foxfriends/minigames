import * as React from "react";
import { render } from "react-dom";
import App from "./App";
import "./index.css";
import validate from "./util/jwt";

try {
  const params = new URLSearchParams(window.location.search);
  const gameId = params.get("game_id");
  const { userId } = await validate(params.get("token"));
  render(
    <App gameId={gameId} userId={userId} />,
    document.querySelector("#app"),
  );
} catch (error) {
  console.error(error);
  render(<div>You don't have access.</div>, document.querySelector("#app"));
}
