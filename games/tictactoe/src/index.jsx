import * as React from "react";
import { render } from "react-dom";
import App from "./App";
import "./index.css";
import validate from "./util/jwt";

async function main() {
  try {
    const params = new URLSearchParams(window.location.search);
    const gameId = params.get("game_id");
    const token = params.get("token");
    const { userId } = await validate(token);
    render(
      <App gameId={gameId} userId={userId} token={token} />,
      document.querySelector("#app"),
    );
  } catch (error) {
    console.error(error);
    render(<div>You don't have access.</div>, document.querySelector("#app"));
  }
}

main();
