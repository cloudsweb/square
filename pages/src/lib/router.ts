import { createRouter, createWebHashHistory } from 'vue-router'

import UserCreate from '../page/UserCreate.vue'
import UserLogin from '../page/UserLogin.vue'
import UserInfo from '../page/UserInfo.vue'

export default createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: "/index" },
    { path: '/users/create', component: UserCreate },
    { path: '/users/login', component: UserLogin },
    { path: '/:user', component: UserInfo },
  ]
})
