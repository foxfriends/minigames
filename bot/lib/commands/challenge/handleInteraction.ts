import { whereEq } from "../../../deps/ramda.ts";
import {
  ButtonStyles,
  DiscordenoInteraction,
  InteractionResponseTypes,
  MessageComponentTypes,
} from "../../../deps/discordeno.ts";
import { userOption } from "../utils.ts";
import { respond, Task, task } from "../../runtime.ts";
import pickRandomGame from "./pickRandomGame.ts";
import createChallenge from "./createChallenge.ts";

export default function handleInteraction({ guildId, user, data }: DiscordenoInteraction): Task {
  // deno-lint-ignore no-explicit-any
  return task(async function* (): AsyncGenerator<Task, void, any> {
    const challengedUserId = userOption(data!.options!.find(whereEq({ name: "user" }))!);
    const challengerUserId = user.id;
    if (challengedUserId === challengerUserId) {
      yield respond({
        type: InteractionResponseTypes.ChannelMessageWithSource,
        private: true,
        data: {
          content: "You cannot challenge yourself, that would be too easy.",
        },
      });
      return;
    }
    const challengedUser = data!.resolved!.users!.get(challengedUserId);
    if (challengedUser?.bot) {
      yield respond({
        type: InteractionResponseTypes.ChannelMessageWithSource,
        private: true,
        data: {
          content: "You cannot challenge a bot, they wouldn't stand a chance.",
        },
      });
      return;
    }
    // deno-fmt-ignore
    const game = `${data?.options?.find(whereEq({ name: "game" }))?.value ?? (yield pickRandomGame(guildId!))}`;
    const gameId: string = yield createChallenge({
      guildId: guildId!,
      challengerId: challengerUserId,
      challengedId: challengedUserId,
      game,
    });
    yield respond({
      type: InteractionResponseTypes.ChannelMessageWithSource,
      data: {
        // deno-fmt-ignore
        content: `<@${challengerUserId}> has challenged <@${challengedUserId}> to a game of **${game}**!\n_Do you accept the challenge?_`,
        allowedMentions: {
          users: [challengerUserId, challengedUserId],
        },
        embeds: [],
        components: [{
          type: 1,
          components: [
            {
              type: MessageComponentTypes.Button,
              style: ButtonStyles.Secondary,
              label: "No",
              customId: "challenge_rejected",
            },
            {
              type: MessageComponentTypes.Button,
              style: ButtonStyles.Success,
              label: "Yes",
              customId: gameId,
            },
          ],
        }],
      },
    });
  });
}
