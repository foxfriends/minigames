import minigame from "@foxfriends/minigames-server-express";
import express from "express";
import { createServer as createViteServer } from "vite";
import chalk from "chalk";
import config from "./vite.config.js";

const {
  NODE_ENV = "development",
  PORT,
  PUBLIC_URL,
  VITE_API_URL,
  VITE_GAME_NAME,
  API_KEY,
} = process.env;

export async function createServer() {
  if (NODE_ENV === "development") {
    const vite = await createViteServer(config);
    return vite.httpServer;
  } else {
    const app = express();
    app.use(express.static("dist"));
    return app;
  }
}

const run = minigame({
  port: PORT,
  apiUrl: VITE_API_URL,
  publicUrl: PUBLIC_URL,
  name: VITE_GAME_NAME,
  apiKey: API_KEY,
});

createServer().then((app) => {
  run(app, (error) => {
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
      process.exit(1);
    }

    console.log(`${chalk.yellow("tictactoe")} is running on port ${PORT}`);
    return () => {
      console.log(`Shutting down ${chalk.yellow("tictactoe")}`);
    };
  });
});
