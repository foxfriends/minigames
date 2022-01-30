import type { ApiRequest } from "./types.ts";

export function describeGameServer(gameName: string): ApiRequest {
  return { path: `/api/v1/servers/${gameName}`, method: "GET" };
}
