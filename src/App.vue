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
@import url('https://fonts.googleapis.com/css2?family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap');

* {
  box-sizing: border-box;
  padding: 0;
  margin: 0;
  font-family: "Noto Sans", Arial, Helvetica, sans-serif;
  font-weight: 400;
}

.app-main {
  /* Global variable for the app */
  --primary-color: hsl(217, 100%, 64%);
  --primary-color-text: hsl(38, 90%, 8%);
  --secondary-color: hsl(34, 100%, 84%);
  --secondary-color-text: hsl(38, 90%, 8%);
  --red-color: hsl(0, 100%, 34%);
  --red-color-text: hsl(0, 0%, 95%);
  --background-color: hsl(15, 40%, 98%);
  --background-color-text: hsl(38, 90%, 8%);

  --s-border-radius: 5px;
  --m-border-radius: 10px;
  --l-border-radius: 20px;

  --s-font-size: 1rem;
  --m-font-size: 1.3rem;
  --l-font-size: 2.2rem;

  --xs-spacing:.5rem;
  --s-spacing:1rem;
  --m-spacing:2rem;
  --l-spacing:4rem;
  --xl-spacing:8rem;

  display: flex;
  height: 100dvh;
  overflow-y: hidden;
  flex-direction: row;
  justify-content: stretch;

  background-color: var(--background-color);
  color: var(--background-color-text);
}

.content {
  width: 100%;
  height: 100%;
}

a:disabled {
  opacity: .7;
  cursor: not-allowed;
}

h1 { font-size: var(--l-font-size); }
h2 { font-size: var(--m-font-size); }
h3 { font-size: var(--s-font-size); }
</style>