import {
  ApplicationCommandOptionTypes,
  ApplicationCommandTypes,
  DiscordenoInteraction,
  InteractionTypes,
} from "../../deps/discordeno.ts";
import type { Command } from "./types.ts";
import { Task } from "../runtime.ts";

import handleInteraction from "./challenge/handleInteraction.ts";
import handleComponentInteraction from "./challenge/handleComponentInteraction.ts";

export const challenge: Command = {
  name: "challenge",
  description: "Challenge a particular user to a 2-player game!",
  type: ApplicationCommandTypes.ChatInput,
  options: [
    {
      type: ApplicationCommandOptionTypes.User,
      name: "user",
      description: "Who to challenge",
      required: true,
    },
    {
      type: ApplicationCommandOptionTypes.String,
      name: "game",
      description: "Which game to play",
      autocomplete: true, // TODO: implement this autocomplete
    },
  ],

  handle(interaction: DiscordenoInteraction): Task | undefined {
    switch (interaction.type) {
      case InteractionTypes.ApplicationCommand:
        return handleInteraction(interaction);
      case InteractionTypes.MessageComponent:
        return handleComponentInteraction(interaction);
    }
  },
};
