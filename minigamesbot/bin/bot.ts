import { createBot, startBot } from "discordeno";
import {
  DISCORD_TOKEN,
  DISCORD_CLIENT_ID,
} from 'lib/config.ts';

const bot = createBot({
  token: DISCORD_TOKEN,
  botId: DISCORD_CLIENT_ID,
  intents: ["GuildMessages", "GuildMessageReactions"],
  events: {},
  cache: { isAsync: false },
});

startBot(bot);
