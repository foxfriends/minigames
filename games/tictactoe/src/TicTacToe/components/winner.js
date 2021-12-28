import { useWinner } from "@minigames/react";
import win from "../win";

export default function useTicTacToeWinner() {
  useWinner((gameState) => {
    const winningCells = win(gameState);
    if (!winningCells) {
      if (gameState.cells.every((cell) => cell.value)) { return null; }
      return;
    }
    const winningSymbol = gameState.cells[winningCells[0]].value;
    const winnerId = gameState[winningSymbol];
    return winnerId;
  });
}
