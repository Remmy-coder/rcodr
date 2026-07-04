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
        reveal: {
          '0%': { opacity: '0', transform: 'translateY(8px) scale(0.98)' },
          '100%': { opacity: '1', transform: 'translateY(0) scale(1)' },
        },
      },
      animation: {
        blink: 'blink 1s step-start infinite',
        glitch: 'glitch 0.6s infinite steps(1, start)',
        reveal: 'reveal 0.5s ease-out forwards',
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
