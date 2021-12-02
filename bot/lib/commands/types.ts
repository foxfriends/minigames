import type { CreateGlobalApplicationCommand, DiscordenoInteraction } from "../../deps/discordeno.ts";
import type { Task } from "../runtime.ts";

interface Command extends CreateGlobalApplicationCommand {
  handleInteraction(interaction: DiscordenoInteraction): Task;
  handleComponentInteraction?(interaction: DiscordenoInteraction): Task;
}

export type { Command };
