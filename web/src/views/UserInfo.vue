<script lang='ts' setup>
import { useUserStore } from '@/lib/store_user';

const user = useUserStore()
</script>

<template>
  <div>
    Welcome to {{ user.name }}'s home!
  </div>
</template>
