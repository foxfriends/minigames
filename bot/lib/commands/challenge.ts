import { whereEq } from "ramda";
import type { Redis } from "redis";
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
import { getGameUrl, invoke, redis, respond, Task, task } from "../runtime.ts";
import * as api from "../api/mod.ts";
import { shame } from "../shame.ts";

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

  handleInteraction({ guildId, user, data }: DiscordenoInteraction): Task {
    // deno-lint-ignore no-explicit-any
    return task(async function* (): AsyncGenerator<Task, void, any> {
      const challengedUserId = userOption(data!.options!.find(whereEq({ name: "user" }))!);
      const challengerUserId = user.id;
      const game = `${data?.options?.find(whereEq({ name: "game" }))?.value ?? pickRandomGame()}`;
      const challengeToken: string = yield createChallenge({
        guildId: guildId!,
        challengerId: challengerUserId,
        challengedId: challengedUserId,
        game,
      });
      const key = crypto.randomUUID();
      yield redis(async (redis: Redis) => {
        await redis.set(key, challengeToken, { ex: 15 * 60 });
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
                customId: key,
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
            content: `${message!.content}\n\nThe challenge was **rejected**... ${shame()}`,
            components: [],
          },
        });
        return;
      }

      const key = data!.customId!;
      const challengeToken: string = yield redis((redis: Redis) => redis.get(key));
      const url: string = yield getGameUrl({ token: challengeToken });

      yield respond({
        type: InteractionResponseTypes.UpdateMessage,
        data: {
          content: `${message!.content}\n\nThe challenge was **accepted**.`,
          components: [
            {
              type: MessageComponentTypes.Button,
              style: ButtonStyles.Primary,
              label: "Let's Play",
              url,
            },
          ],
        },
      });
    });
  },
};
