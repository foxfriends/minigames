import { defineConfig } from "vite";
import { config } from "dotenv";

config()

const base = process.env.PUBLIC_BASE_PATH;

export default defineConfig({
  server: { hmr: false },
  base,
});
