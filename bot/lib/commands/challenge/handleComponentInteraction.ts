import {
  ButtonStyles,
  DiscordenoInteraction,
  DiscordenoMessage,
  InteractionResponseTypes,
  MessageComponents,
  MessageComponentTypes,
} from "../../../deps/discordeno.ts";
import { getGameUrl, respond, Task, task } from "../../runtime.ts";
import { shame } from "../../shame.ts";

function alreadyResponded(message: DiscordenoMessage, userId: bigint): boolean {
  if (message.authorId === userId) return true;
  // A sketchy way to check but... what more can we do than check message content?
  return message.content.includes(`<@${userId}> has`);
}

export default function handleComponentInteraction(
  { message, user, data }: DiscordenoInteraction,
): Task {
  // deno-lint-ignore no-explicit-any
  return task(async function* (): AsyncGenerator<Task, void, any> {
    const participants: bigint[] = message!.mentionedUserIds ?? [];

    if (!participants.includes(user.id)) {
      yield respond({
        type: InteractionResponseTypes.ChannelMessageWithSource,
        private: true,
        data: {
          content: "You are not a participant in this game. Why not challenge someone yourself?",
        },
      });
      return;
    }

    if (alreadyResponded(message!, user.id)) {
      yield respond({
        type: InteractionResponseTypes.ChannelMessageWithSource,
        private: true,
        data: {
          content: "You have already responded to this challenge. Wait for the others!",
        },
      });
      return;
    }

    if (data!.customId === "challenge_rejected") {
      yield respond({
        type: InteractionResponseTypes.UpdateMessage,
        data: {
          content: `${message!.content}\n\n<@${user.id}> has **rejected**... ${shame()}`,
          components: [],
        },
      });
      return;
    }

    const gameId = data!.customId!;
    const url: string = yield getGameUrl(gameId);

    const allResponded = participants
      .filter((participant) => participant !== user.id)
      .every((participant) => alreadyResponded(message!, participant));

    let components: MessageComponents | undefined;
    if (allResponded) {
      components = [{
        type: 1,
        components: [{
          type: MessageComponentTypes.Button,
          style: ButtonStyles.Link,
          label: "Let's Play",
          url,
        }],
      }];
    }

    yield respond({
      type: InteractionResponseTypes.UpdateMessage,
      data: {
        content: `${message!.content}\n\n<@${user.id}> has **accepted**.`,
        components,
      },
    });
  });
}
