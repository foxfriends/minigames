import type {
  CreateGlobalApplicationCommand,
  DiscordenoInteraction,
} from "../../deps/discordeno.ts";
import type { Task } from "../runtime.ts";

interface Command extends CreateGlobalApplicationCommand {
  handle(interaction: DiscordenoInteraction): Task | undefined;
}

export type { Command };
