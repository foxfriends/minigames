import {
  DiscordenoInteraction,
  InteractionDataOption,
  InteractionResponseTypes,
} from "../../../deps/discordeno.ts";
import { respond, Task, task } from "../../runtime.ts";
import listGames from "../tasks/listGames.ts";

export default function autocompleteGameName(
  { guildId }: DiscordenoInteraction,
  { value }: InteractionDataOption,
): Task {
  return task(async function* (): AsyncGenerator<Task, void, string[]> {
    const games = yield listGames(guildId!);

    yield respond({
      type: InteractionResponseTypes.ApplicationCommandAutocompleteResult,
      data: {
        choices: games
          .filter((game) => game.startsWith(value as string))
          .slice(0, 25)
          .map((name) => ({ name, value: name })),
      },
    });
  });
}
