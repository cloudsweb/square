import { createRouter, createWebHashHistory } from 'vue-router'

import UserCreate from '@/views/UserCreate.vue'
import UserLogin from '@/views/UserLogin.vue'
import UserInfo from '@/views/UserInfo.vue'

export default createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: "/index" },
    { path: '/users/create', component: UserCreate },
    { path: '/users/login', component: UserLogin },
    { path: '/:user', component: UserInfo },
  ]
})
