import {
  Bot,
  DiscordenoInteraction,
  DiscordenoInteractionResponse,
  sendInteractionResponse,
} from "../deps/discordeno.ts";
import type { ApiRequest, Client } from "./api/mod.ts";

export type RuntimeConfig = {
  webUrl: string;
  invoke: Client;
};

export type RuntimeContext = {
  bot: Bot;
  interaction: DiscordenoInteraction;
};

export type Context = RuntimeConfig & RuntimeContext;

export type Task = (context: Context) => Promise<unknown>;
export type TaskGenerator = () => AsyncGenerator<Task, unknown, unknown>;
export type Runner = (context: RuntimeContext, task: Task) => unknown;

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

export function respond(options: DiscordenoInteractionResponse): Task {
  return ({ bot, interaction: { id, token } }) => sendInteractionResponse(bot, id, token, options);
}

export function getGameUrl(token: string): Task {
  return ({ webUrl }) => Promise.resolve(`${webUrl}/play?game_id=${token}`);
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

export function runtime(config: RuntimeConfig): Runner {
  return (context: RuntimeContext, task: Task) => task({ ...context, ...config });
}
