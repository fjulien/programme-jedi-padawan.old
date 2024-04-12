/** @type {import('tailwindcss').Config} */
module.exports = {
    content:  ["./src/**/*.rs"],
    theme: {
      theme: {
        extend: {
          fontFamily: {
            montserrat: ["Montserrat", "sans-serif"]
          },
        },
      },
    },
    plugins: [],
    mode: "jit",
  }