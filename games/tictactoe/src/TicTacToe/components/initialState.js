import { useInitialState, useGameInfo } from "@foxfriends/minigames-client-react";
import { version } from "../../../package.json";
import { X } from "../constants";

export default function useInitialTicTacToeState() {
  const { players } = useGameInfo();

  useInitialState(() => {
    const x = Math.floor(Math.random() * 2);
    return {
      version,
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
}
