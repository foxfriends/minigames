import type { ApiRequest } from "./types.ts";

export function listGames(): ApiRequest {
  return { path: "/api/v1/games", method: "GET" };
}
