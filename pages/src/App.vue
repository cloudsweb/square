<template>
  <div>
    <div id='nav'>
      <router-link to='/'> Home</router-link>
      <span v-if="state.alias != null">{{ state.alias }}</span>
      <span v-else>
        <router-link to='/users/create'>Sign Up</router-link>/
        <router-link to='/users/login'>Sign In</router-link>
      </span>
    </div>
    <router-view />
  </div>
</template>

<script lang="ts">
import { defineComponent, provide, reactive, readonly } from 'vue'
import State from './lib/state'

export default defineComponent({
  setup() {
    const state = reactive(new State())
    const setUser = (alias: string, token: string) => {
      console.log('setUser', alias, token)
      state.alias = alias
      state.token = token
    }
    provide('state', state)
    provide('setUser', setUser)
    return { state }
  },
})
</script>
