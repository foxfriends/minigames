import { Task, task } from "../../runtime.ts";
import listGames from "./listGames.ts";

export default function pickRandomGame(guildId: bigint): Task {
  return task(async function* (): AsyncGenerator<Task, string, string[]> {
    const games: string[] = yield listGames(guildId);
    if (games.length === 0) {
      throw new Error("No games are available to be played right now.");
    }
    return games[Math.floor(Math.random() * games.length)];
  });
}
