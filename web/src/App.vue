<script lang='ts' setup>
import { useUserStore } from './lib/stores';

const user = useUserStore()
</script>

<template>
  <div>
    <div id='nav'>
      <router-link to='/'> Home</router-link>
      <span v-if="user.alias != null">
        {{ user.alias }} /
        <a href='' @click="user.logout">Sign out</a>
      </span>
      <span v-else>
        <router-link to='/users/create'>Sign Up</router-link> /
        <router-link to='/users/login'>Sign In</router-link>
      </span>
    </div>
    <router-view />
  </div>
</template>
