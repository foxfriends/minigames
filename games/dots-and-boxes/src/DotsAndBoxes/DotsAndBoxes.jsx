import React, { createContext, useContext } from "react";
import { useGameInfo, useGameState } from "@foxfriends/minigames-client-react";
import useInitialDotsAndBoxesState from "./components/initialState";
import useDotsAndBoxesWinner from "./components/winner";
import * as lens from "../util/lens";
import * as line from "./line";
import append from "../util/append";
import useFns from "./fns";

const DotsAndBoxesContext = createContext();

export function useDotsAndBoxes() {
  return useContext(DotsAndBoxesContext);
}

export default function DotsAndBoxes({ children }) {
  const { getLineFaces, getFaceLines } = useFns();
  useInitialDotsAndBoxesState();
  const winner = useDotsAndBoxesWinner();

  const [gameState, setGameState] = useGameState();
  const { me, players, options: { size = 5 } = {} } = useGameInfo();

  function updateGameState(fn) {
    setGameState(fn(gameState));
  }

  const isMyTurn = gameState?.turn === me;

  function addLine(line, state) {
    return lens.mod(lens.prop("lines"), append(line), state);
  }

  function setTurn(turn, state) {
    return lens.set(lens.prop("turn"), turn, state);
  }

  function scoreFace(face, scorer, state) {
    return lens.set(
      lens.compose(lens.prop("boxes"), lens.nth(face)),
      scorer,
      state,
    );
  }

  function drawLine(drawn) {
    let updated = addLine(drawn, gameState);

    let scored = false;
    const lineFaces = getLineFaces(drawn);
    for (const face of lineFaces) {
      const faceLines = getFaceLines(face);
      console.log(faceLines);
      const closed = faceLines.every((a) =>
        updated.lines.some((b) => line.eq(a, b)),
      );
      if (closed) {
        updated = scoreFace(face, me, updated);
        scored = true;
      }
    }

    const them = players?.find(({ id }) => id !== me).id;
    updated = scored ? updated : setTurn(them, updated);
    setGameState(updated);
  }

  const dotsAndBoxes = {
    gameState,
    me,
    size,
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
