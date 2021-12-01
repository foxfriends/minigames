export interface ApiRequest extends RequestInit {
  path: string;
}

export interface Client {
  (request: ApiRequest): Promise<Response>;
}

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

type ClientConfig = {
  apiUrl: string;
};

export function client({ apiUrl }: ClientConfig): Client {
  return ({ path, ...init }: ApiRequest): Promise<Response> => {
    return fetch(apiUrl + path, init);
  };
}
