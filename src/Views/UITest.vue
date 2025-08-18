<script setup lang="ts">
import Button from "../Components/Button.vue";
</script>

<template>
  <section class="center">
    <div class="wrapper">
      <section>
        <h1>lorem ipsum dolor sit amet consectetur</h1>
        <h2>elit cillum feugait eu placerat aliquyam</h2>
        <h3>gubergren aliquid ut aliquam doming justo</h3>
      </section>
      <Button @click="console.log('button A')">A</Button>
      <Button variant="red" expand @click="console.log('button Test')">Test</Button>
      <Button variant="secondary" @click="console.log('Signed in with google')">Sign in with google.com</Button>
      <Button @click="console.log('Disabled button')" disabled>Disabled</Button>
    </div>
  </section>
</template>

<style scoped>
.center {
  display: grid;
  width: 100%;
  height: 100%;

  align-items: center;
  justify-content: center;
}

.center .wrapper {
  display: flex;
  flex-direction: column;
  justify-content: start;
  align-items: start;
  gap: var(--s-spacing);
}
</style>