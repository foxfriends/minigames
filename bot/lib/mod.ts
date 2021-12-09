import { whereEq } from "../deps/ramda.ts";
import {
  Bot,
  createApplicationCommand,
  createEventHandlers,
  DiscordenoInteraction,
  InteractionResponseTypes,
  InteractionTypes,
  sendInteractionResponse,
} from "../deps/discordeno.ts";
import { blue, green, red } from "../deps/colors.ts";
import { commands } from "./commands/mod.ts";
import { client } from "./api/mod.ts";
import { runtime } from "./runtime.ts";

type Config = {
  apiUrl: string;
  webUrl: string;
  guild?: bigint;
};

function prepareMinigamesBot(bot: Bot, {
  apiUrl,
  webUrl,
  guild,
}: Config) {
  const invoke = client({ apiUrl });
  const run = runtime({ invoke, webUrl });

  const { ready, interactionCreate } = bot.events;
  bot.events = Object.assign(
    bot.events,
    createEventHandlers({
      async ready(bot: Bot, payload, rawPayload) {
        for (const command of commands) {
          createApplicationCommand(bot, command, guild);
        }

        await ready(bot, payload, rawPayload);
      },

      async interactionCreate(
        bot: Bot,
        interaction: DiscordenoInteraction,
      ) {
        if (
          interaction.type === InteractionTypes.ApplicationCommand &&
          interaction.data
        ) {
          const command = commands.find(
            whereEq({ name: interaction.data.name }),
          );
          if (command) {
            try {
              const task = command.handleInteraction(interaction);
              await run({ bot, interaction }, task);
            } catch (error) {
              const interactionName = blue(interaction.data!.name!);
              console.error(
                `Interaction ${interactionName} has ${red("failed")}:`,
                error,
              );
              await sendInteractionResponse(
                bot,
                interaction.id,
                interaction.token,
                {
                  type: InteractionResponseTypes.ChannelMessageWithSource,
                  private: true,
                  data: {
                    content:
                      `Sorry, looks like there was a problem: **${error.message}**`,
                  },
                },
              );
            }
            return;
          }
        } else if (
          interaction.type === InteractionTypes.MessageComponent &&
          interaction.message && interaction.data
        ) {
          const command = commands.find(
            whereEq({ name: interaction.message!.interaction!.name }),
          );
          if (command) {
            try {
              const task = command.handleComponentInteraction?.(interaction);
              if (task) await run({ bot, interaction }, task);
            } catch (error) {
              const interactionName = blue(
                interaction.message!.interaction!.name,
              );
              const component = green(`"${interaction.data!.customId}"`);
              // deno-fmt-ignore
              console.error(`Interaction ${interactionName} component ${component} has ${red("failed")}:`, error);
              await sendInteractionResponse(
                bot,
                interaction.id,
                interaction.token,
                {
                  type: InteractionResponseTypes.ChannelMessageWithSource,
                  private: true,
                  data: {
                    content:
                      `Sorry, looks like there was a problem: **${error.message}**`,
                  },
                },
              );
            }
            return;
          }
        }

        await interactionCreate(bot, interaction);
      },
    }),
  );
}

export { prepareMinigamesBot };
