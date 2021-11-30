import { blue, green, red } from "fmt/colors.ts";
import { whereEq } from "ramda";
import { Bot, createApplicationCommand, InteractionTypes } from "discordeno";
import { DISCORD_DEBUG_GUILD } from "./config.ts";
import { commands } from "./commands/mod.ts";

function prepareMinigamesBot(bot: Bot) {
  const { ready, interactionCreate } = bot.events;
  bot.events = {
    ...bot.events,
    async ready(bot, payload, rawPayload) {
      for (const command of commands) {
        createApplicationCommand(bot, command, DISCORD_DEBUG_GUILD);
      }

      await ready?.(bot, payload, rawPayload);
    },

    async interactionCreate(bot, interaction) {
      if (
        interaction.type === InteractionTypes.ApplicationCommand &&
        interaction.data
      ) {
        const command = commands.find(whereEq({ name: interaction.data.name }));
        if (command) {
          try {
            await command.handle(bot, interaction);
          } catch (error) {
            const interactionName = blue(interaction.data!.name!);
            console.error(
              `Interaction ${interactionName} has ${red("failed")}: ${error}`,
            );
          }
          return;
        }
      } else if (
        interaction.type === InteractionTypes.MessageComponent &&
        interaction.message && interaction.data
      ) {
        const command = commands.find(whereEq({ name: interaction.message!.interaction!.name }));
        if (command) {
          try {
            await command.handleComponent?.(bot, interaction);
          } catch (error) {
            const interactionName = blue(interaction.message!.interaction!.name);
            const component = green(`"${interaction.data!.customId}"`);
            console.error(
              `Interaction ${interactionName} component ${component} has ${
                red("failed")
              }: ${error}`,
            );
          }
          return;
        }
      }

      await interactionCreate?.(bot, interaction);
    },
  };
}

export { prepareMinigamesBot };
