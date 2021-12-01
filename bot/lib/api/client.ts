import { ApiRequest } from "./types.ts";

export interface Client {
  (request: ApiRequest): Promise<Response>;
}

type ClientConfig = {
  apiUrl: string;
};

export function client({ apiUrl }: ClientConfig): Client {
  return ({ path, ...init }: ApiRequest): Promise<Response> => {
    return fetch(apiUrl + path, init);
  };
}
