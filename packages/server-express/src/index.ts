import type { Server } from "connect";
import fetch from "node-fetch";

export type Options = {
  name: string;
  port: number;
  apiUrl: string;
  apiKey: string;
  publicUrl: string;
};

export default function minigame({
  name,
  port,
  apiUrl,
  apiKey,
  publicUrl,
}: Options) {
  return function run(
    app: Server,
    callback: (error?: Error) => () => {} | undefined,
  ) {
    let onclose: () => {} | undefined;

    const server = app.listen(port, async () => {
      try {
        await fetch(`${apiUrl}/games/${name}`, {
          method: "POST",
          headers: { Authorization: `Bearer ${apiKey}` },
          body: JSON.stringify({ url: publicUrl }),
        });
        onclose = callback();
      } catch (error) {
        callback(error);
      }
    });

    async function shutdown() {
      if (typeof onclose === "function") {
        onclose();
      }
      await fetch(`${apiUrl}/games/tictactoe`, {
        method: "DELETE",
        headers: { Authorization: `Bearer ${apiKey}` },
      });
      server.close();
      process.exit(0);
    }

    process.on("SIGTERM", shutdown);
    process.on("SIGINT", shutdown);
  };
}
