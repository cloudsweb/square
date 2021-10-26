<template>
  <n-space vertical>
    <n-input v-model:value="user.alias" type="text" placeholder="Username" />
    <n-input v-model:value="user.password" type="password" placeholder="Password" />
    <n-button @click="submit">Login</n-button>
  </n-space>
</template>

<script lang="ts">
import { defineComponent, reactive } from 'vue'
import { useRouter } from 'vue-router'

class UserInfo {
  alias?: string
  password?: string
}

export default defineComponent({
  setup() {
    const user = reactive(new UserInfo())
    const router = useRouter()
    const submit = async () => {
      console.log(user)
      // 1. check
      // 2. submit
      try {
        const resp = await fetch('/api/users/login', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(user),
        })

        const result = await resp.json()
        
        // TODO: check user.alias
        if (result.token != null && result.token != '') {
          console.log(`token: ${result.token}`)
        }

        if (user.alias != null && user.alias != '') {
          router.push(`/${user.alias}`)
        }
      } catch {}
    }
    return { user, submit }
  },
})
</script>
