import {
  ApplicationCommandOptionTypes,
  ApplicationCommandTypes,
  DiscordenoInteraction,
  InteractionTypes,
} from "../../../deps/discordeno.ts";
import type { Command } from "../types.ts";
import { Task } from "../../runtime.ts";

import handleInteraction from "./handleInteraction.ts";
import handleAutocomplete from "./handleAutocomplete.ts";

export const score: Command = {
  name: "score",
  description: "Get the score between some players",
  type: ApplicationCommandTypes.ChatInput,
  options: [
    {
      type: ApplicationCommandOptionTypes.User,
      name: "user",
      description: "Who to check score of",
      required: true,
    },
    {
      type: ApplicationCommandOptionTypes.User,
      name: "against",
      description: "Who to compare score against, if not yourself",
    },
    {
      type: ApplicationCommandOptionTypes.String,
      name: "game",
      description: "Which game to compare score of",
      autocomplete: true,
    },
  ],

  handle(interaction: DiscordenoInteraction): Task | undefined {
    switch (interaction.type) {
      case InteractionTypes.ApplicationCommand:
        return handleInteraction(interaction);
      // case InteractionTypes.MessageComponent:
      //   return handleComponentInteraction(interaction);
      case InteractionTypes.ApplicationCommandAutocomplete:
        return handleAutocomplete(interaction);
    }
  },
};
