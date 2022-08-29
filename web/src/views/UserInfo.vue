<script lang='ts' setup>
import { useUserStore } from '@/lib/stores';

const user = useUserStore()
</script>

<template>
  <div>
    Welcome to {{ user.name }}'s home!
  </div>
</template>
