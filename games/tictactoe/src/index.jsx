import * as React from "react";
import { render } from "react-dom";
import App from "./App";
import "./index.css";
import validate from "./util/jwt";

async function confirmUser(token) {
  try {
    const { userId } = await validate(token);
    return userId;
  } catch (error) {
    console.warn('User could not be determined:', error);
    return null;
  }
}

async function main() {
  const params = new URLSearchParams(window.location.search);
  const gameId = params.get("game_id");
  const token = params.get("token");
  const userId = await confirmUser(token);
  render(
    <App gameId={gameId} userId={userId} token={token} />,
    document.querySelector("#app"),
  );
}

main();
