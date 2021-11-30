import { config } from "dotenv";

const {
  DISCORD_TOKEN,
  DISCORD_PREFIX,
  ...env
} = config({ safe: true });

export { DISCORD_PREFIX, DISCORD_TOKEN };

export const DISCORD_CLIENT_ID = BigInt(env.DISCORD_CLIENT_ID);
export const DISCORD_DEBUG_GUILD = env.DISCORD_DEBUG_GUILD
  ? BigInt(env.DISCORD_DEBUG_GUILD)
  : undefined;
