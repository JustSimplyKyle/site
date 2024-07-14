/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
        fontFamily: {
          rubik: ['Rubik'],
        },
        colors: {
        'surface': '#151515',
        'deep-background': '#0E0E0E',
        'secondary-surface': '#2F2F2F',
        'green': '#CDE347',
        'red': '#C92B45',
        'black': "#232E3D",
        'light-blue': '#7CAED3',
        'tinted-yellow': '#D3E950',
        'title': '#F2F2F2',
        'body': '#D3D4D4',
        'hover': '#50C3F6',
      },
      animation: {
        floatUp: 'floatUp 0.5s ease-out forwards'
      },
    },
  },
  plugins: [],
};
