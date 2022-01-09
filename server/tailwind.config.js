module.exports = {
  content: ["./src/**/*.{html,rs,js}"],
  theme: {
    extend: {
      colors: {
        // Colors are mostly taken off discord website
        background: {
          floating: "#18191c",
          default: "#36393F",
          secondary: "#2F3136",
        },
        blurple: {
          default: "#5865f2",
          hover: "#3C45A5",
        },
        text: {
          heading: "#ffffff",
          body: "#b9bbbe",
        },
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
