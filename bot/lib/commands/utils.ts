import { ApplicationCommandOptionTypes, InteractionDataOption } from "../../deps/discordeno.ts";

type Handler<T> = (option: InteractionDataOption) => T;

export function optional<T>(
  handler: Handler<T>,
): (option: InteractionDataOption | undefined) => T | undefined {
  return (option: InteractionDataOption | undefined) => {
    if (!option?.value) return undefined;
    return handler(option);
  };
}

export function userOption(option: InteractionDataOption): bigint {
  if (option.type !== ApplicationCommandOptionTypes.User) {
    throw new TypeError(`option ${option.name} is not a User option`);
  }
  return BigInt(option.value as string);
}
