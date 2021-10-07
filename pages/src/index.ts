import { createApp } from 'vue'
import router from './lib/router'
import ui from './lib/ui'
import App from './App.vue'

createApp(App)
  .use(router)
  .use(ui)
  .mount('#app')
