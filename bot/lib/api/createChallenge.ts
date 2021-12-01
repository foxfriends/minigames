import type { ApiRequest } from "./types.ts";

export type CreateChallenge = {
  guildId: BigInt;
  challengerId: BigInt;
  challengedId: BigInt;
  game?: string;
};

export function createChallenge(
  { guildId, challengerId, challengedId, game }: CreateChallenge,
): ApiRequest {
  return {
    path: "/challenge",
    method: "POST",
    body: JSON.stringify({
      guildId: guildId.toString(),
      challengerId: challengerId.toString(),
      challengedId: challengedId.toString(),
      game,
    }),
    headers: { "Content-Type": "application/json" },
  };
}
