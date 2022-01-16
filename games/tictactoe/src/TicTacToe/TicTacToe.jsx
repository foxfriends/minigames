import React, { createContext, useContext } from "react";
import { useGameInfo, useGameState } from "@foxfriends/minigames-client-react";
import useInitialTicTacToeState from "./components/initialState";
import useTicTacToeWinner from "./components/winner";
import { X, O } from "./constants";
import * as lens from "../util/lens";
import * as fn from "../util/fn";
import win from "./win";

const TicTacToeContext = createContext();

export function useTicTacToe() {
  return useContext(TicTacToeContext);
}

export default function TicTacToe({ children }) {
  useInitialTicTacToeState();
  const winner = useTicTacToeWinner();

  const [gameState, setGameState] = useGameState();
  const { me } = useGameInfo();

  function updateGameState(fn) {
    setGameState(fn(gameState));
  }

  let mine;
  let theirs;
  if (gameState) {
    if (gameState.x === me) {
      mine = X;
      theirs = O;
    } else if (gameState.o === me) {
      mine = O;
      theirs = X;
    }
  }

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

  const winningCells = gameState && win(gameState);
  const tictactoe = {
    gameState,
    mine,
    theirs,
    myTurn,
    select,
    winningCells,
    winner,
  };

  return (
    <TicTacToeContext.Provider value={tictactoe}>
      {children}
    </TicTacToeContext.Provider>
  );
}
