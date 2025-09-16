module.exports = {
  mode: "jit",
  content: {
    files: ["src/**/*.rs", "index.html"],
  },
  darkMode: "media", // 'media' or 'class'
  theme: {
    extend: {
      keyframes: {
        blink: {
          '0%, 50%, 100%': { opacity: '1' },
          '25%, 75%': { opacity: '0' },
        },
        glitch: {
          '0%, 100%': { transform: 'none', opacity: '1' },
          '20%': { transform: 'skew(-5deg)', opacity: '0.75' },
          '40%': { transform: 'skew(5deg)', opacity: '0.5' },
          '60%': { transform: 'skew(-3deg)', opacity: '0.8' },
          '80%': { transform: 'skew(3deg)', opacity: '0.6' },
        },
      },
      animation: {
        blink: 'blink 1s step-start infinite',
        glitch: 'glitch 1s infinite steps(1, start)',
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
