import express from 'express';
import { createServer as createViteServer } from 'vite';
import chalk from 'chalk';
import fetch from 'node-fetch';
import config from './vite.config.js';

const { NODE_ENV = 'development', PORT, PUBLIC_URL, API_URL, API_KEY } = process.env;

export async function createServer() {
  if (NODE_ENV === 'development') {
    const vite = await createViteServer(config);
    return vite.httpServer;
  } else {
    const app = express();
    app.use(express.static('dist'));
    return app;
  }
}

createServer().then((app) => {
  const server = app.listen(PORT, async () => {
    console.log(`${chalk.yellow('tictactoe')} is running on port ${PORT}`);
    await fetch(`${API_URL}/game/tictactoe`, {
      method: 'POST',
      headers: { Authorization: `Bearer ${API_KEY}` },
      body: JSON.stringify({ url: PUBLIC_URL }),
    });
  });

  async function shutdown() {
    console.log(`Shutting down ${chalk.yellow('tictactoe')}`);
    await fetch(`${API_URL}/game/tictactoe`, {
      method: 'DELETE',
      headers: { Authorization: `Bearer ${API_KEY}` },
    });
    server.close();
    process.exit(0);
  };

  process.on('SIGTERM', shutdown);
  process.on('SIGINT', shutdown);
});
