import React, { createContext, useContext } from "react";
import { useMinigame, useInitialState } from "@minigames/react";
import { X, O } from "./constants";
import * as lens from "../util/lens";
import * as fn from "../util/fn";

const TicTacToeContext = createContext();

export function useTicTacToe() {
  return useContext(TicTacToeContext);
}

export default function TicTacToe({ children }) {
  const { gameState, setGameState, players, me } = useMinigame();

  function updateGameState(fn) {
    setGameState(fn(gameState));
  }

  useInitialState(() => {
    const x = Math.floor(Math.random() * 2);
    return {
      turn: X,
      x: players[x].id,
      o: players[1 - x].id,
      // The board is arranged this way:
      //   0 1 2
      //   3 4 5
      //   6 7 8
      cells: [
        { value: null },
        { value: null },
        { value: null },
        { value: null },
        { value: null },
        { value: null },
        { value: null },
        { value: null },
        { value: null },
      ],
    };
  });

  const mine = gameState && gameState.x === me ? X : O;
  const theirs = gameState && gameState.x === me ? O : X;
  const myTurn = gameState?.turn === mine;

  function cellLens(cellIndex) {
    return lens.compose(
      lens.prop("cells"),
      lens.nth(cellIndex),
      lens.prop("value"),
    );
  }

  const turnLens = lens.prop("turn");

  function select(cellIndex) {
    updateGameState(
      fn.compose(
        lens.set(cellLens(cellIndex))(mine),
        lens.set(turnLens)(theirs),
      ),
    );
  }

  const tictactoe = {
    gameState,
    mine,
    theirs,
    myTurn,
    select,
  };

  return (
    <TicTacToeContext.Provider value={tictactoe}>
      {children}
    </TicTacToeContext.Provider>
  );
}
