module.exports = {
  root: true,
  parser: "@typescript-eslint/parser",
  parserOptions: { sourceType: "module" },
  env: {
    es2021: true,
    node: true,
  },
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/eslint-recommended",
    "plugin:@typescript-eslint/recommended",
    "prettier",
  ],
};
