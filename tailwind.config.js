module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        base: 'var(--color-base)',
        border: 'var(--color-border)',
        primary: 'var(--color-primary)',
        primaryText: 'var(--color-primary-text)',
        info: 'var(--color-info)',
        secondary: 'var(--color-secondary)',
        danger: 'var(--color-danger)',
      }
    }
  }
}
