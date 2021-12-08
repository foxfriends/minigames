import * as React from "react";
import { createContext, useContext, useState } from "react";

const DEFAULT = {
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

const GameStateContext = createContext(DEFAULT);

export function useGameState() {
  return useContext(GameStateContext);
}

export default function GameStateProvider({ children }) {
  const [state, setState] = useState(DEFAULT);

  return (
    <GameStateContext.Provider value={state}>
      {children}
    </GameStateContext.Provider>
  );
}
