import type { ApiRequest } from "./types.ts";

export type ListGamesResponse = string[];

export function listGames(guildId: bigint): ApiRequest {
  return { path: `/api/v1/games?guild_id=${guildId}`, method: "GET" };
}
