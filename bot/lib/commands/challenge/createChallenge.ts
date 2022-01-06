import { invoke, Task, task } from "../../runtime.ts";
import * as api from "../../api/mod.ts";

export default function createChallenge(params: api.CreateChallenge): Task {
  return task(async function* (): AsyncGenerator<Task, string, Response> {
    const response: Response = yield invoke(api.createChallenge(params));
    if (response.status === 200) {
      const { gameId } = await response.json();
      return gameId;
    } else {
      throw await response.json();
    }
  });
}
