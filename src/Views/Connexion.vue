<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {useRouter} from "vue-router";

const router = useRouter();

const server = ref("");
const port = ref(993);
const token = ref("");
const email = ref("");

function TokenConnect() {
  invoke("token_connect", {
    server: server.value,
    port: port.value,
    email: email.value,
    token: token.value
  }).then(value => { console.log("TokenConnect Result : ", value) })

  router.push("/")
}

</script>

<template>
<div class="token_form">
  <div class="field">
    <label for="server">Server: </label>
    <input type="text" id="server" v-model="server">
  </div>
  <div class="field">
    <label for="port">Port: </label>
    <input type="number" max="65535" id="port" v-model="port">
  </div>
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