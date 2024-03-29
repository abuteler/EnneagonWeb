/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      animation: {
        'blink': 'blink .9s linear infinite',
        'xplode': 'xplode .5s linear 1'
      },
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'gradient-conic':
          'conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))',
      },
      colors: {
        'vectris-floor': 'rgb(70 110 130)',
        'vectris-upnext-border': 'rgb(77 150 187)',
      },
      keyframes: {
        blink: {
          '50%': { opacity: '0' },
        },
        xplode: {
          '50%': { opacity: '0' },
        }
      }
    },
  },
  plugins: [],
}