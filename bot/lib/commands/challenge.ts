import { whereEq } from "ramda";
import {
  ApplicationCommandOptionTypes,
  ApplicationCommandTypes,
  ButtonStyles,
  DiscordenoInteraction,
  InteractionResponseTypes,
  MessageComponentTypes,
} from "discordeno";
import { userOption } from "./utils.ts";
import { pickRandomGame } from "../games.ts";
import type { Command } from "./types.ts";
import { invoke, respond, Task, task } from "../runtime.ts";
import * as api from "../api.ts";

function createChallenge(params: api.CreateChallenge): Task {
  return task(async function* (): AsyncGenerator<Task, string, Response> {
    const response: Response = yield invoke(api.createChallenge(params));
    const { token } = await response.json();
    return token;
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
      autocomplete: true,
    },
    {
      type: ApplicationCommandOptionTypes.String,
      name: "game",
      description: "Which game to play",
      autocomplete: true,
      // choices: [/* TODO: list all known games */],
    },
  ],

  handleInteraction({ id, guildId, token, user, data }: DiscordenoInteraction): Task {
    // deno-lint-ignore no-explicit-any
    return task(async function* (): AsyncGenerator<Task, void, any> {
      const challengedUserId = userOption(data!.options!.find(whereEq({ name: "user" }))!);
      const challengerUserId = user.id;
      const game = `${data?.options?.find(whereEq({ name: "game" }))?.value ?? pickRandomGame()}`;
      const _challengeToken: string = yield createChallenge({
        guildId: guildId!,
        challengerId: challengerUserId,
        challengedId: challengedUserId,
        game,
      });
      yield respond(id, token, {
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
                customId: "challenge_accepted",
              },
            ],
          }],
        },
      });
    });
  },

  handleComponentInteraction({ id, token, message, data }: DiscordenoInteraction): Task {
    return task(async function* (): AsyncGenerator<Task> {
      const result = data!.customId === "challenge_rejected" ? "**rejected**" : "**accepted**";
      yield respond(id, token, {
        type: InteractionResponseTypes.UpdateMessage,
        data: {
          // deno-fmt-ignore
          content: `${message!.content}\n\nThe challenge was ${result}... _but this feature is not yet implemented._`,
          components: [],
        },
      });
    });
  },
};
