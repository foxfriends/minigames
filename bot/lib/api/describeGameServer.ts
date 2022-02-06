import type { ApiRequest } from "./types.ts";

export type DescribeGameServerResponse = {
  name: string;
  imageUrl?: string;
};

export function describeGameServer(gameName: string): ApiRequest {
  return { path: `/api/v1/servers/${gameName}`, method: "GET" };
}
