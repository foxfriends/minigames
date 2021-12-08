module.exports = {
  root: true,
  parserOptions: { sourceType: "module" },
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
  extends: ["eslint:recommended", "plugin:react/recommended", "prettier"],
  overrides: [
    {
      files: "server.js",
      env: {
        browser: false,
        node: true,
      },
    },
  ],
};
