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
    const submit = () => {
      console.log(user)
      // 1. check
      // 2. submit
    }
    return { user, submit }
  },
})
</script>
