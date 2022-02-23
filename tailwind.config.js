module.exports = {
  content: ["./index.html", "css/**/*.css"],
  theme: {
    colors: {
      neonBlue: '#2EAFFF',
      neonPink: '#CE48FF',
      chambray: '#394483',
      scampi: '#6570AC',
      moonRaker: '#C2C6F4',
      stratos: {
        dark: '#000A39',
        light: '#000D54',
      },
      white: '#ffffff'
    },
    fontFamily: {
      'mono': ['Space Mono', 'ui-monospace', 'SFMono-Regular', 'Menlo', 'Monaco', 'Consolas', "Liberation Mono", "Courier New", 'monospace'],
      'sans': ['Loos Wide', 'ui-sans-serif', 'system-ui', '-apple-system', 'BlinkMacSystemFont', "Segoe UI", 'Roboto', "Helvetica Neue", 'Arial', "Noto Sans", 'sans-serif', "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"],
    },
  },
  plugins: [],
}
