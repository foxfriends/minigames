import createMinigame from "@foxfriends/minigames-server-express";
import express from "express";
import { createServer as createViteServer } from "vite";
import chalk from "chalk";
import config from "./vite.config.js";

const {
  NODE_ENV = "development",
  PORT,
  VITE_API_URL,
  VITE_GAME_NAME,
  SECRET_KEY,
} = process.env;

const minigame = createMinigame({
  port: PORT,
  apiUrl: VITE_API_URL,
  name: VITE_GAME_NAME,
  secretKey: SECRET_KEY,
});

export async function createServer() {
  if (NODE_ENV === "development") {
    const vite = await createViteServer({
      ...config,
      server: { ...config.server, middlewareMode: "html" },
    });
    const app = express();
    app.use(minigame.middleware());
    app.use(vite.middlewares);
    return app;
  } else {
    const app = express();
    app.use(minigame.middleware());
    app.use(express.static("dist"));
    return app;
  }
}

createServer().then((app) => {
  minigame.run(app, (error) => {
    if (error) {
      if (error.name === "FetchError") {
        console.log(
          `${chalk.red(
            "Error",
          )}: Could not connect to the main server at ${chalk.cyan(
            VITE_API_URL,
          )}. Ensure the server is running and that the ${chalk.green(
            "VITE_API_URL",
          )} environment variable is set correctly.`,
        );
      } else {
        console.error(error.message);
      }
    }

    console.log(`${chalk.yellow("tictactoe")} is running on port ${PORT}`);
    return () => {
      console.log(`Shutting down ${chalk.yellow("tictactoe")}`);
    };
  });
});
