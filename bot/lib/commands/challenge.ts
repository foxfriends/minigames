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
import type { Command } from "./types.ts";
import { getGameUrl, invoke, respond, Task, task } from "../runtime.ts";
import * as api from "../api/mod.ts";
import { shame } from "../shame.ts";

function pickRandomGame(): Task {
  return task(async function* (): AsyncGenerator<Task, string, Response> {
    const response: Response = yield invoke(api.listGames());
    if (response.status === 200) {
      const games = await response.json();
      if (games.length === 0) {
        throw new Error("No games are available to be played right now.");
      }
      return games[Math.floor(Math.random() * games.length)];
    } else {
      throw await response.json();
    }
  });
}

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
      const game = `${data?.options?.find(whereEq({ name: "game" }))?.value ?? (yield pickRandomGame())}`;
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
            content: `${message!.content}\n\nThe challenge was **rejected**... ${shame()}`,
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
