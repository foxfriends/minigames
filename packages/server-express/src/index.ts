import type { Server } from "connect";
import fetch from "node-fetch";

export type Options = {
  name: string;
  port: number;
  apiUrl: string;
  secretKey: string;
};

export default function minigame({ name, port, apiUrl, secretKey }: Options) {
  return function run(
    app: Server,
    callback: (error?: unknown) => (() => unknown) | undefined,
  ) {
    let onclose: (() => unknown) | undefined;

    const server = app.listen(port, async () => {
      try {
        await fetch(`${apiUrl}/api/v1/servers/${name}/available`, {
          method: "POST",
          headers: { "X-Api-Key": secretKey },
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
      await fetch(`${apiUrl}/api/v1/servers/${name}/unavailable`, {
        method: "POST",
        headers: { "X-Api-Key": secretKey },
      });
      server.close();
      process.exit(0);
    }

    process.on("SIGTERM", shutdown);
    process.on("SIGINT", shutdown);
  };
}
