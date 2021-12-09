import { config } from "../../deps/dotenv.ts";

config({ safe: true, export: true });

const {
  DISCORD_TOKEN,
  DISCORD_PREFIX,
  API_URL,
  WEB_URL,
  ...env
} = Deno.env.toObject();

export { API_URL, DISCORD_PREFIX, DISCORD_TOKEN, WEB_URL };

export const DISCORD_CLIENT_ID = BigInt(env.DISCORD_CLIENT_ID);
export const DISCORD_DEBUG_GUILD = env.DISCORD_DEBUG_GUILD
  ? BigInt(env.DISCORD_DEBUG_GUILD)
  : undefined;
