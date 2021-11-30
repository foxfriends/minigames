import { ApplicationCommandOptionTypes, InteractionDataOption } from "discordeno";

export function userOption(option: InteractionDataOption): bigint {
  if (option.type !== ApplicationCommandOptionTypes.User) {
    throw new TypeError(`option ${option.name} is not a User option`);
  }
  return BigInt(option.value as string);
}