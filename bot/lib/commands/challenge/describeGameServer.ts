import { invoke, Task, task } from "../../runtime.ts";
import * as api from "../../api/mod.ts";

export type GameServer = {
  name: string;
  imageUrl?: string;
};

export default function describeGameServer(gameName: string): Task {
  return task(async function* (): AsyncGenerator<Task, GameServer, Response> {
    const response: Response = yield invoke(api.describeGameServer(gameName));
    if (response.status === 200) {
      return await response.json() as GameServer;
    } else {
      throw await response.json();
    }
  });
}
