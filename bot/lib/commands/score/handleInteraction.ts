import { whereEq } from "../../../deps/ramda.ts";
import { DiscordenoInteraction, InteractionResponseTypes } from "../../../deps/discordeno.ts";
import { optional, userOption } from "../utils.ts";
import { respond, Task, task } from "../../runtime.ts";
import getScore, { GetScore } from "./getScore.ts";

export default function handleInteraction({ guildId, user, data }: DiscordenoInteraction): Task {
  // deno-lint-ignore no-explicit-any
  return task(async function* (): AsyncGenerator<Task, void, any> {
    const users = [
      userOption(data!.options!.find(whereEq({ name: "user" }))!),
      optional(userOption)(data!.options!.find(whereEq({ name: "against" }))) ?? user.id,
    ];
    const differentUsers = new Set(users);
    if (differentUsers.size <= 1) {
      yield respond({
        type: InteractionResponseTypes.ChannelMessageWithSource,
        private: true,
        data: {
          content: "You need at least 2 different players to compare.",
        },
      });
      return;
    }
    const includesBots = users
      .map((user) => data!.resolved!.users!.get(user))
      .some((user) => user?.bot);
    if (includesBots) {
      yield respond({
        type: InteractionResponseTypes.ChannelMessageWithSource,
        private: true,
        data: {
          content: "Bots cannot play, so they do not have scores.",
        },
      });
      return;
    }
    // deno-fmt-ignore
    const game = data?.options?.find(whereEq({ name: "game" }))?.value as string;
    const { score, totalGames }: GetScore = yield getScore({
      guildId: guildId!,
      users,
      game,
    });
    yield respond({
      type: InteractionResponseTypes.ChannelMessageWithSource,
      data: {
        // deno-fmt-ignore
        content: `${game ?? "All Games"} - <@${users[0]}> vs. <@${users[1]}>: ${score[0]} to ${score[1]} (${totalGames} games played)`,
        allowedMentions: { users },
      },
    });
  });
}
