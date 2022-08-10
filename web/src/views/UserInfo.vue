<template>
  <div>
    Welcome to {{ user.alias }}'s home!
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, inject } from 'vue'
import type State from '@/lib/state'

export default defineComponent({
  setup() {
    // TODO!: get id from params
    const state = inject<State>('state')
    const user = computed(() => ({
      alias: state?.alias
    }))
    return { user }
  },
})
</script>
