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
          hover: "#292B2F",
          light: "#b9bbbe",
        },
        divider: {
          dark: "#040405",
          light: "#b9bbbe",
        },
        blurple: {
          default: "#5865f2",
          hover: "#3C45A5",
        },
        text: {
          heading: "#ffffff",
          input: "#f6f6f7",
          body: "#b9bbbe",
          dark: "#36393F",
        },
        switch: {
          unchecked: "#72767D",
          toggle: "#FFFFFF",
        },
      },
      boxShadow: {
        switch: "0 2px 4px rgba(0, 0, 0, 0.3)",
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
