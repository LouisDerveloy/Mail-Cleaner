<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";


</script>
<template>
  <main class="app-main">
    <aside>
      <RouterLink to="/">H</RouterLink>
    </aside>
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
}


.app-main aside {
  padding: 1rem;
  background: grey;

  height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: center;
}

.app-main aside RouterLink {
  font-size: 20px;
  text-decoration: none;
}

.content {
  width: 100%;
  height: 100%;
}

.app-main {
  display: flex;
  height: 100%;
  flex-direction: row;
  justify-content: stretch;
}
</style>