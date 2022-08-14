import { fileURLToPath, URL } from 'node:url'

import { defineConfig, loadEnv, type ConfigEnv } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default ({ mode }: ConfigEnv) => {
  const env = {...loadEnv(mode, process.cwd()+'/..'), ...process.env}
  console.log("connect api to", env.VITE_API_URL)

  return defineConfig({
    plugins: [vue()],
    resolve: {
      alias: {
        '@': fileURLToPath(new URL('./src', import.meta.url))
      }
    },
    server: {
      port: 7080,
      proxy: {
        '/api': {
          target: env.VITE_API_URL ?? "http://127.0.0.1:7079", // TODO: support localhost
          rewrite: (path) => path.replace(/^\/api/, ''),
        }
      },
    },
  })
}
