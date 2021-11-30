import { whereEq } from "ramda";
import {
  ApplicationCommandOptionTypes,
  ApplicationCommandTypes,
  ButtonStyles,
  InteractionResponseTypes,
  MessageComponentTypes,
  sendInteractionResponse,
} from "discordeno";
import { userOption } from "./utils.ts";
import type { Command } from "./types.ts";

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

  async handle(bot, { id, token, user, data }) {
    const challengedUserId = userOption(
      data!.options!.find(whereEq({ name: "user" }))!,
    );
    const challengerUserId = user.id;
    const game = data!.options!.find(whereEq({ name: "game" }))?.value ?? "Chess";
    await sendInteractionResponse(bot, id, token, {
      type: InteractionResponseTypes.ChannelMessageWithSource,
      data: {
        content:
          `<@${challengerUserId}> has challenged <@${challengedUserId}> to a game of **${game}**!\n_Do you accept the challenge?_`,
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
  },

  async handleComponent(bot, { id, token, message, data }) {
    const result = data!.customId === "challenge_accepted" ? "**accepted**" : "**rejected**";
    await sendInteractionResponse(bot, id, token, {
      type: InteractionResponseTypes.UpdateMessage,
      data: {
        content: `${
          message!.content
        }\n\nThe challenge was ${result}... _but this feature is not yet implemented._`,
        components: [],
      },
    });
  },
};
