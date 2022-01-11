module.exports = {
  root: true,
  parserOptions: { sourceType: "module" },
  parser: "@typescript-eslint/parser",
  env: {
    browser: true,
    es2021: true,
  },
  plugins: ["react"],
  settings: {
    react: {
      version: "detect",
    },
  },
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/eslint-recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:react/recommended",
    "prettier",
  ],
  overrides: [
    {
      files: "server.ts",
      env: {
        browser: false,
        node: true,
      },
    },
  ],
};
