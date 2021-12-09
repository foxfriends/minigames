import { Bot, createBot, startBot } from "../../deps/discordeno.ts";
import { bold, yellow } from "../../deps/colors.ts";
import { prepareMinigamesBot } from "../../lib/mod.ts";
import {
  API_URL,
  DISCORD_CLIENT_ID,
  DISCORD_DEBUG_GUILD,
  DISCORD_TOKEN,
  WEB_URL,
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
  });

  await prepareMinigamesBot(bot, {
    apiUrl: API_URL,
    webUrl: WEB_URL,
    guild: DISCORD_DEBUG_GUILD,
  });

  await startBot(bot);
}

await main();
