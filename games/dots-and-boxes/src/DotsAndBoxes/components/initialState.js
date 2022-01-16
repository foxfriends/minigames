import {
  useInitialState,
  useGameInfo,
} from "@foxfriends/minigames-client-react";
import range from "../../util/range";
import { version } from "../../../package.json";

export default function useInitialDotsAndBoxesState() {
  const {
    players,
    options: { size = 5 },
  } = useGameInfo();

  useInitialState(() => {
    return {
      version,
      size,
      turn: players[Math.floor(Math.random() * 2)].id,
      lines: [],
      boxes: range(size ** 2).map(() => null),
    };
  });
}
