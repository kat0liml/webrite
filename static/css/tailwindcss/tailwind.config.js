/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../../html/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('daisyui'),
  ],
}

