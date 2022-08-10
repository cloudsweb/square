<template>
  <n-space vertical>
    <n-input v-model:value="user.alias" type="text" placeholder="Username" />
    <n-input v-model:value="user.password" type="password" placeholder="Password" />
    <n-button @click="submit">Login</n-button>
  </n-space>
</template>

<script lang="ts">
import { defineComponent, inject, reactive } from 'vue'
import { useRouter } from 'vue-router'
import type State from '@/lib/state'

class UserInfo {
  alias?: string
  password?: string
}

export default defineComponent({
  setup() {
    const user = reactive(new UserInfo())
    const state = inject<State>('state')
    const router = useRouter()
    const submit = async () => {
      console.log(user)
      // 1. check
      // 2. submit

      state?.login(user.alias ?? '', user.password ?? '')

      if (user.alias != null && user.alias != '') {
        router.push(`/${user.alias}`)
      }
    }
    return { user, submit }
  },
})
</script>
