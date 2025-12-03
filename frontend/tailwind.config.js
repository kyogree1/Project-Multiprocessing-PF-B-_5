/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        pink: {
          primary: '#ff6b9d',
          dark: '#ff1744',
          light: '#fff5f7',
        }
      }
    },
  },
  plugins: [],
}
