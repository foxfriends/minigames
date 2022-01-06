import { invoke, Task, task } from "../../runtime.ts";
import * as api from "../../api/mod.ts";

export default function pickRandomGame(): Task {
  return task(async function* (): AsyncGenerator<Task, string, Response> {
    const response: Response = yield invoke(api.listGames());
    if (response.status === 200) {
      const games = await response.json();
      if (games.length === 0) {
        throw new Error("No games are available to be played right now.");
      }
      return games[Math.floor(Math.random() * games.length)];
    } else {
      throw await response.json();
    }
  });
}
