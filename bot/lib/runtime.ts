import type { Redis } from "redis";
import type { ApiRequest, Client } from "./api.ts";
import { Bot, DiscordenoInteractionResponse, sendInteractionResponse } from "discordeno";

export type RuntimeConfig = {
  redis: Redis;
  invoke: Client;
};

export type Context = RuntimeConfig & {
  bot: Bot;
};

export type Task = (context: Context) => Promise<unknown>;
export type TaskGenerator = () => AsyncGenerator<Task, unknown, unknown>;
export type Runner = (bot: Bot, task: Task) => unknown;

function chain(transform: (value: unknown) => Promise<Task>, task: Task): Task {
  return async (context: Context) => {
    const response = await task(context);
    const result = await transform(response);
    return result(context);
  };
}

export function invoke(request: ApiRequest): Task {
  return ({ invoke }) => invoke(request);
}

export function respond(
  id: bigint,
  token: string,
  options: DiscordenoInteractionResponse,
): Task {
  return ({ bot }) => sendInteractionResponse(bot, id, token, options);
}

export function task(generator: TaskGenerator): Task {
  function next(iter: AsyncIterator<Task, unknown, unknown>) {
    return async (input: unknown): Promise<Task> => {
      const yielded = await iter.next(input);
      if (yielded.done) return () => Promise.resolve(yielded.value);
      return chain(next(iter), yielded.value);
    };
  }

  return async (context: Context): Promise<unknown> => {
    const iter = await generator();
    const task = await next(iter)(undefined);
    return task(context);
  };
}

export function runtime(context: RuntimeConfig): Runner {
  return (bot: Bot, task: Task) => task({ ...context, bot });
}
