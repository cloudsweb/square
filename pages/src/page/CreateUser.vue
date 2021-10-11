<template>
  <n-space vertical>
    <n-input v-model:value="user.alias" type="text" placeholder="Username" />
    <n-input v-model:value="user.name" type="text" placeholder="Nickname" />
    <n-input v-model:value="user.password" type="text" placeholder="Password" />
    <n-input v-model:value="user.email" type="text" placeholder="Email" />
    <n-input v-model:value="user.desc" type="textarea" placeholder="Basic Textarea" />
    <n-button @click="submit">Submit</n-button>
  </n-space>
</template>

<script lang="ts">
import { defineComponent, reactive } from 'vue'
import { useRouter } from 'vue-router'

class UserCreate {
  alias?: string
  name?: string
  password?: string
  email?: string
  desc?: string
}

export default defineComponent({
  setup() {
    const user = reactive(new UserCreate())
    const router = useRouter()
    const submit = () => {
      console.log(user)
      // 1. check
      // 2. submit
      fetch('/api/users/create', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(user),
      }).then(() => {
        // TODO: check user.alias
        if (user.alias != null && user.alias != '') {
          router.push(`/${user.alias}`)
        }
        // jump to ...
      }).catch(() => {
        // show error
      })
    }
    return { user, submit }
  },
})
</script>
