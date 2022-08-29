<script lang='ts' setup>
import { reactive } from 'vue'
import { useRouter } from 'vue-router'
import { useUserStore, UserCreateInfo } from '@/lib/store_user'

const router = useRouter()
const user = useUserStore()
const login_info = reactive(new UserCreateInfo)
async function submit() {
  await user.signup(login_info)

  if (user.alias != null && user.alias != '') {
    router.push(`/${user.alias}`)
  }
}
</script>

<template>
  <n-space vertical>
    <n-input v-model:value="login_info.alias" type="text" placeholder="Username" />
    <n-input v-model:value="login_info.name" type="text" placeholder="Nickname" />
    <n-input v-model:value="login_info.password" type="password" placeholder="Password" />
    <n-input v-model:value="login_info.email" type="text" placeholder="Email" />
    <n-input v-model:value="login_info.desc" type="textarea" placeholder="Basic Textarea" />
    <n-button @click="submit">Submit</n-button>
  </n-space>
</template>
