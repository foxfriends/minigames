module.exports = {
  content: ["./src/**/*.{html,rs,js}"],
  theme: {
    extend: {
      colors: {
        // Colors are mostly taken off discord website
        "background-floating": "#18191c",
        "background-default": "#36393F",
        "background-secondary": "#2F3136",
        "blurple": "#5865f2",
        "text-heading": "#ffffff",
        "text-body": "#b9bbbe",
      },
      fontFamily: {
        sans: ['Whitney', 'Open Sans', 'San Francisco', 'Ubuntu', 'Arial', 'sans-serif'],
      },
      borderRadius: {
        sm: "0.2rem"
      }
    },
  },
  plugins: [],
};
