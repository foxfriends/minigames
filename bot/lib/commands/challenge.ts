import { whereEq } from "../../deps/ramda.ts";
import {
  ApplicationCommandOptionTypes,
  ApplicationCommandTypes,
  ButtonStyles,
  DiscordenoInteraction,
  InteractionResponseTypes,
  MessageComponentTypes,
} from "../../deps/discordeno.ts";
import { userOption } from "./utils.ts";
import { pickRandomGame } from "../games.ts";
import type { Command } from "./types.ts";
import { getGameUrl, invoke, respond, Task, task } from "../runtime.ts";
import * as api from "../api/mod.ts";
import { shame } from "../shame.ts";

function createChallenge(params: api.CreateChallenge): Task {
  return task(async function* (): AsyncGenerator<Task, string, Response> {
    const response: Response = yield invoke(api.createChallenge(params));
    if (response.status === 200) {
      const { gameId } = await response.json();
      return gameId;
    } else {
      throw await response.json();
    }
  });
}

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

  handleInteraction({ guildId, user, data }: DiscordenoInteraction): Task {
    // deno-lint-ignore no-explicit-any
    return task(async function* (): AsyncGenerator<Task, void, any> {
      const challengedUserId = userOption(
        data!.options!.find(whereEq({ name: "user" }))!,
      );
      const challengerUserId = user.id;
      const game = `${
        data?.options?.find(whereEq({ name: "game" }))?.value ??
          pickRandomGame()
      }`;
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
  },

  handleComponentInteraction({ message, data }: DiscordenoInteraction): Task {
    // deno-lint-ignore no-explicit-any
    return task(async function* (): AsyncGenerator<Task, void, any> {
      if (data!.customId === "challenge_rejected") {
        yield respond({
          type: InteractionResponseTypes.UpdateMessage,
          data: {
            content: `${
              message!.content
            }\n\nThe challenge was **rejected**... ${shame()}`,
            components: [],
          },
        });
        return;
      }

      const gameId = data!.customId!;
      const url: string = yield getGameUrl(gameId);

      yield respond({
        type: InteractionResponseTypes.UpdateMessage,
        data: {
          content: `${message!.content}\n\nThe challenge was **accepted**.`,
          components: [{
            type: 1,
            components: [{
              type: MessageComponentTypes.Button,
              style: ButtonStyles.Link,
              label: "Let's Play",
              url,
            }],
          }],
        },
      });
    });
  },
};
