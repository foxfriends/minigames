import { DISCORD_CLIENT_ID } from "./bot/config.ts";

console.log(
  `https://discord.com/api/oauth2/authorize?client_id=${DISCORD_CLIENT_ID}&scope=bot%20applications.commands&permissions=10240`,
);
