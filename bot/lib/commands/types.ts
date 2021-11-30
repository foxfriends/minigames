import type { Bot, CreateGlobalApplicationCommand, DiscordenoInteraction } from "discordeno";

interface Command extends CreateGlobalApplicationCommand {
  handle(bot: Bot, interaction: DiscordenoInteraction): unknown;
  handleComponent?(bot: Bot, interaction: DiscordenoInteraction): unknown;
}

export type { Command };
