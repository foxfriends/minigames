import * as React from "react";
import { render } from "react-dom";
import Minigame from "@foxfriends/minigames-client-react";
import DotsAndBoxes from "./DotsAndBoxes";
import App from "./App";
import "./index.css";

render(
  <Minigame
    name={import.meta.env.VITE_GAME_NAME}
    apiUrl={import.meta.env.VITE_API_URL}
    socketUrl={import.meta.env.VITE_SOCKET_URL}
  >
    <DotsAndBoxes>
      <App />
    </DotsAndBoxes>
  </Minigame>,
  document.querySelector("#app"),
);
