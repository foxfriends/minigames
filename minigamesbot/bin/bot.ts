import { bold, yellow } from "fmt/colors.ts";
import { createBot, startBot } from "discordeno";
import { DISCORD_CLIENT_ID, DISCORD_TOKEN } from "lib/config.ts";
import { prepareMinigamesBot } from "lib/mod.ts";

function main() {
  const bot = createBot({
    token: DISCORD_TOKEN,
    botId: DISCORD_CLIENT_ID,
    intents: [],
    events: {
      ready(_bot, { user }) {
        console.log(`${bold(yellow(user.username))} is running!`);
      },
    },
    cache: { isAsync: false },
  });
  prepareMinigamesBot(bot);
  startBot(bot);
}

main();
