import { config } from "dotenv";

const {
  DISCORD_TOKEN,
  DISCORD_PREFIX,
  ...env
} = config({ safe: true });

export {
  DISCORD_TOKEN,
  DISCORD_PREFIX,
};

export const DISCORD_CLIENT_ID = BigInt(env.DISCORD_CLIENT_ID);
