<template>
  <div>
    <div id='nav'>
      <router-link to='/'> Home</router-link>
      <router-link to='/users/create'>Sign Up</router-link>/
      <router-link to='/users/login'>Sign In</router-link>
    </div>
    <router-view />
  </div>
</template>
