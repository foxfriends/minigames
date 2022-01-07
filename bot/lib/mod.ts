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

function getCommandName(interaction: DiscordenoInteraction): string {
  switch (interaction.type) {
    case InteractionTypes.ApplicationCommand:
    case InteractionTypes.ApplicationCommandAutocomplete:
      return interaction.data!.name!;
    case InteractionTypes.MessageComponent:
      return interaction.message!.interaction!.name!;
    default: // unsupported ones we can just not find
      return "";
  }
}

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

      async interactionCreate(bot: Bot, interaction: DiscordenoInteraction) {
        const commandName = getCommandName(interaction);
        const command = commands.find(whereEq({ name: commandName }));
        if (!command) {
          return interactionCreate(bot, interaction);
        }

        try {
          const task = command.handle(interaction);
          if (task) await run({ bot, interaction }, task);
        } catch (error) {
          const commandMsg = `Interaction ${blue(commandName)}`;
          const componentMsg = interaction.type === InteractionTypes.MessageComponent
            ? `component ${green(interaction.data!.customId!)}`
            : "";
          const descriptor = [commandMsg, componentMsg].filter(Boolean).join(" ");
          console.error(`${descriptor} has ${red("failed")}:`, error);
          await sendInteractionResponse(
            bot,
            interaction.id,
            interaction.token,
            {
              type: InteractionResponseTypes.ChannelMessageWithSource,
              private: true,
              data: {
                content: `Sorry, looks like there was a problem: **${error.message}**`,
              },
            },
          );
        }
      },
    }),
  );
}

export { prepareMinigamesBot };
