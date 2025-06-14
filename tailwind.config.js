/** @type {import("tailwindcss").Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,jsx,ts,tsx}"],
  theme: {
    extend: {
      fontFamily: {
        monsterrat: "Montserrat, serif",
      },
      colors: {
        "dark-blue-1": "#001e78",
        "green-1": "#00925d",
      },
    },
  },
  plugins: [],
};
