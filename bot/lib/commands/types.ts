import type { CreateApplicationCommand, DiscordenoInteraction } from "../../deps/discordeno.ts";
import type { Task } from "../runtime.ts";

interface Command extends CreateApplicationCommand {
  handle(interaction: DiscordenoInteraction): Task | undefined;
}

export type { Command };
