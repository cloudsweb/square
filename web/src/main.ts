import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './lib/router'
import naiveui from 'naive-ui'

// import './assets/main.css'

createApp(App)
  .use(createPinia())
  .use(router)
  .use(naiveui)
  .mount('#app')
