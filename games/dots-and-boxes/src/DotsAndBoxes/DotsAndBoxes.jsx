import React, { createContext, useContext } from "react";
import { useGameInfo, useGameState } from "@foxfriends/minigames-client-react";
import useInitialDotsAndBoxesState from "./components/initialState";
import useDotsAndBoxesWinner from "./components/winner";
import * as lens from "../util/lens";
import * as fn from "../util/fn";

const DotsAndBoxesContext = createContext();

export function useDotsAndBoxes() {
  return useContext(DotsAndBoxesContext);
}

export default function DotsAndBoxes({ children }) {
  useInitialDotsAndBoxesState();
  const winner = useDotsAndBoxesWinner();

  const [gameState, setGameState] = useGameState();
  const { me } = useGameInfo();

  function updateGameState(fn) {
    setGameState(fn(gameState));
  }

  const isMyTurn = gameState?.turn === me;

  function drawLine(start, end) {
    throw new Error("Unimplemented");
  }

  const dotsAndBoxes = {
    gameState,
    isMyTurn,
    winner,
    drawLine,
  };

  return (
    <DotsAndBoxesContext.Provider value={dotsAndBoxes}>
      {children}
    </DotsAndBoxesContext.Provider>
  );
}
