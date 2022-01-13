import { invoke, Task, task } from "../../runtime.ts";
import * as api from "../../api/mod.ts";

export default function listGames(guildId: bigint): Task {
  return task(async function* (): AsyncGenerator<Task, string[], Response> {
    const response: Response = yield invoke(api.listGames(guildId));
    if (response.status === 200) {
      return await response.json() as string[];
    } else {
      throw await response.json();
    }
  });
}
