/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
      files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
      extend: {},
    },
    plugins: [],
    purge: {
      enabled: true,
      content: ["./src/**/*.html"],
    },
    mode: "jit",
  }