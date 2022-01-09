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
        divider: {
          dark: "#040405"
        },
        blurple: {
          default: "#5865f2",
          hover: "#3C45A5",
        },
        text: {
          heading: "#ffffff",
          input: "#f6f6f7",
          body: "#b9bbbe",
        },
      },
      fontFamily: {
        sans: ['Whitney', 'Open Sans', 'San Francisco', 'Ubuntu', 'Arial', 'sans-serif'],
        mono: ['Source Code Pro', 'Ubuntu Mono', 'Courier New'],
      },
      borderRadius: {
        sm: "0.2rem"
      }
    },
  },
  plugins: [],
};
