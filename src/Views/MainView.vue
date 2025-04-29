<script setup lang="ts">
import {useRouter} from "vue-router";

const router = useRouter()

function GoToAnalyse() {
  router.push("/analyse")
}

</script>

<template>
  <div class="container">
    <h1>Home</h1>
    <RouterLink to="analyse">Analyse my email</RouterLink>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100vh;
}
button {
  margin-top: 2rem;
  padding: .5rem;
  border-radius: .5rem;
  border: 3px solid;
}
</style>