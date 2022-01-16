module.exports = {
  extends: ["stylelint-config-standard", "stylelint-config-prettier"],
  rules: {
    "custom-property-pattern": /\w+--(\w-)*\w+/,
  },
};
