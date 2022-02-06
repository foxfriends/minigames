import { invoke, Task, task } from "../../runtime.ts";
import * as api from "../../api/mod.ts";

export type GetScore = api.GetScoreResponse;

export default function getScore(params: api.GetScoreRequest): Task {
  return task(async function* (): AsyncGenerator<Task, api.GetScoreResponse, Response> {
    const response: Response = yield invoke(api.getScore(params));
    if (response.status === 200) {
      return response.json();
    } else {
      throw await response.json();
    }
  });
}
