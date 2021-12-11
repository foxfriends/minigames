import type { ApiRequest } from "./types.ts";

export function listGames(): ApiRequest {
  return { path: "/games", method: "GET" };
}
