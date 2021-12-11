import express from "express";
import { createServer as createViteServer } from "vite";
import chalk from "chalk";
import fetch from "node-fetch";
import config from "./vite.config.js";

const {
  NODE_ENV = "development",
  PORT,
  PUBLIC_URL,
  VITE_API_URL,
  API_KEY,
} = process.env;

async function tryFetch(...params) {
  try {
    const response = await fetch(...params);
    if (response.status !== 200) {
      const { code, message, data } = await response.json();
      throw new Error(message);
    }
    return response;
  } catch (error) {
    if (error.name === 'FetchError') {
      console.log(`${chalk.red('Error')}: Could not connect to the main server at ${chalk.cyan(VITE_API_URL)}. Ensure the server is running and that the ${chalk.green('VITE_API_URL')} environment variable is set correctly.`);
    } else {
      console.error(error.message);
    }
    process.exit(1);
  }
}

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

createServer()
  .then(async (app) => {
    const server = await app.listen(PORT, async () => {
      console.log(`${chalk.yellow("tictactoe")} is running on port ${PORT}`);
      await tryFetch(`${VITE_API_URL}/games/tictactoe`, {
        method: "POST",
        headers: { Authorization: `Bearer ${API_KEY}` },
        body: JSON.stringify({ url: PUBLIC_URL }),
      });
    });

    async function shutdown() {
      console.log(`Shutting down ${chalk.yellow("tictactoe")}`);
      await tryFetch(`${VITE_API_URL}/games/tictactoe`, {
        method: "DELETE",
        headers: { Authorization: `Bearer ${API_KEY}` },
      });
      server.close();
      process.exit(0);
    }

    process.on("SIGTERM", shutdown);
    process.on("SIGINT", shutdown);
  });
