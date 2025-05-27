<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";

const token = ref("");
const email = ref("");

function TokenConnect() {
  invoke("token_connect", {
    email: email.value,
    token: token.value
  })
}

</script>

<template>
<div class="token_form">
  <div class="field">
    <label for="email">Email: </label>
    <input type="text" id="email" v-model="email">
  </div>
  <div class="field">
    <label for="token">Token: </label>
    <input type="password" id="token" v-model="token">
  </div>

  <button @click="TokenConnect">Connect</button>
</div>
</template>

<style scoped>

.token_form {
  margin: 2rem 2rem;
}

.token_form .field {
  display: block;
  margin-top: .5rem;
}

.token_form button {
  margin-top: 1rem;
  margin-left: 2rem;
  padding: .5rem 2rem;
}
</style>