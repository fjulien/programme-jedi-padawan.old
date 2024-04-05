/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
      files: ["*.html", "./src/**/*.rs"],
    },
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
    purge: {
      enabled: true,
      content: ["./src/**/*.html"],
    },
    mode: "jit",
  }