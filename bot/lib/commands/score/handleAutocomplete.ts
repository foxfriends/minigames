import { whereEq } from "../../../deps/ramda.ts";
import { DiscordenoInteraction } from "../../../deps/discordeno.ts";
import { Task } from "../../runtime.ts";
import autocompleteGameName from "../autocomplete/gameName.ts";

export default function handleAutocomplete(interaction: DiscordenoInteraction): Task | undefined {
  const option = interaction.data!.options!.find(whereEq({ focused: true }));
  switch (option?.name) {
    case "game":
      return autocompleteGameName(interaction, option);
  }
}
