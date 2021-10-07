import { createRouter, createWebHashHistory } from 'vue-router'

import CreateUser from '../page/CreateUser.vue'

export default createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: "/index" },
    { path: '/users/create', component: CreateUser }
  ]
})
