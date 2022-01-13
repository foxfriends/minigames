import { whereEq } from "../../../deps/ramda.ts";
import {
  DiscordenoInteraction,
  InteractionDataOption,
  InteractionResponseTypes,
} from "../../../deps/discordeno.ts";
import { respond, Task, task } from "../../runtime.ts";
import listGames from "./listGames.ts";

function autocompleteGameName(
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

export default function handleAutocomplete(interaction: DiscordenoInteraction): Task | undefined {
  const option = interaction.data!.options!.find(whereEq({ focused: true }));
  switch (option?.name) {
    case "game":
      return autocompleteGameName(interaction, option);
  }
}
