/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      colors: {
        'surface': '#151515',
        'title': '#F2F2F2',
        'body': '#D3D4D4',
        'hover': '#50C3F6',
      },
    },
  },
  plugins: [],
};
