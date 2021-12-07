module.exports = {
  root: true,
  parserOptions: { sourceType: 'module' },
  env: {
    browser: true,
    es2021: true,
  },
  plugins: ['react'],
  extends: [
    'eslint:recommended',
    'plugin:react/recommended',
  ],

  overrides: [{
    files: 'server.js',
    env: {
      browser: false,
      node: true,
    },
  }],
};
