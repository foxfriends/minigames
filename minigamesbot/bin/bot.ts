import { yellow, bold } from "fmt/colors.ts";
import { createBot, startBot } from "discordeno";
import {
  DISCORD_TOKEN,
  DISCORD_CLIENT_ID,
} from 'lib/config.ts';
import { prepareMinigamesBot } from 'lib/mod.ts';

function main() {
  const bot = createBot({
    token: DISCORD_TOKEN,
    botId: DISCORD_CLIENT_ID,
    intents: [],
    events: {
      ready(bot, { user, guilds }) {
        console.log(`${bold(yellow(user.username))} is running!`);
      },
    },
    cache: { isAsync: false },
  });
  prepareMinigamesBot(bot);
  startBot(bot);
}

main();
