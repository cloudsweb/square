import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
export default defineConfig({
  plugins: [ vue() ],
  server: {
    port: 7080,
    proxy: {
      '/api': {
        target: "http://localhost:7079",
        rewrite: (path) => path.replace(/^\/api/, ''),
      }
    },
  },
})
