import { Bot, createBot, startBot } from "discordeno";
import { bold, yellow } from "fmt/colors.ts";
import { prepareMinigamesBot } from "lib/mod.ts";
import {
  API_URL,
  DISCORD_CLIENT_ID,
  DISCORD_DEBUG_GUILD,
  DISCORD_TOKEN,
  REDIS_URL,
} from "./config.ts";

async function main() {
  const bot = createBot({
    token: DISCORD_TOKEN,
    botId: DISCORD_CLIENT_ID,
    intents: [],
    events: {
      ready(_bot: Bot, { user }) {
        console.log(`${bold(yellow(user.username))} is running!`);
      },
    },
    cache: { isAsync: false },
  });

  await prepareMinigamesBot(bot, {
    redisUrl: REDIS_URL,
    apiUrl: API_URL,
    guild: DISCORD_DEBUG_GUILD,
  });

  await startBot(bot);
}

await main();
