import type { ApiRequest } from "./types.ts";

export type GetScoreRequest = {
  guildId: BigInt;
  users: BigInt[];
  game?: string;
};

export type GetScoreResponse = {
  score: number[];
  totalGames: number;
};

export function getScore(
  { guildId, users, game }: GetScoreRequest,
): ApiRequest {
  const params = new URLSearchParams([
    ["guild_id", guildId.toString()],
    ...users.map((user) => ["users", user.toString()]),
    ["game", game],
  ].filter((pair): pair is string[] => !!pair[1]));
  return {
    path: `/api/v1/score?${params.toString()}`,
    method: "GET",
  };
}
