<script setup lang="ts">
import {RouterView, useRouter} from "vue-router";

import {listen} from "@tauri-apps/api/event";

const router = useRouter();
listen("open-login-page", () => {
  console.log("open-login-page event received")
  router.push("/user/connexion")
})

</script>
<template>
  <main class="app-main">
    <section class="content">
      <RouterView />
    </section>
  </main>
</template>
<style>
* {
  box-sizing: border-box;
  padding: 0;
  margin: 0;
  font-family: Arial, Helvetica, sans-serif;
}

.app-main {
  display: flex;
  height: 100dvh;
  overflow-y: hidden;
  flex-direction: row;
  justify-content: stretch;
}

.content {
  width: 100%;
  height: 100%;
}

button, a {
  padding: .25rem 1rem;
  margin: .25rem;
  border: none;
  text-decoration: none;
  background: #45b2f1;
  cursor: pointer;
  text-transform: uppercase;
  color: rgb(20, 20, 20);
}

button:disabled, a:disabled {
  opacity: .7;
  cursor: not-allowed;
}
</style>