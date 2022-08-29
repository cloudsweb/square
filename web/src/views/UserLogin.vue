<script lang='ts' setup>
import { reactive } from 'vue';
import { useRouter } from 'vue-router'
import { useUserStore, UserLoginInfo } from '@/lib/store_user'

const login_info = reactive(new UserLoginInfo)
const router = useRouter()
const user = useUserStore()

async function submit() {
  console.log(login_info)
  await user.login(login_info)
  // 1. check
  // 2. submit
  if (user.alias != null && user.alias != '') {
    router.push(`/${user.alias}`)
  }
}
</script>

<template>
  <n-space vertical>
    <n-input v-model:value="login_info.alias" type="text" placeholder="Username" />
    <n-input v-model:value="login_info.password" type="password" placeholder="Password" />
    <n-button @click="submit">Login</n-button>
  </n-space>
</template>
