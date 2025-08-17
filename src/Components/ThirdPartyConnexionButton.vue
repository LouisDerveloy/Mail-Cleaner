<script setup lang="ts">

defineProps({
  ico: String,
  label: String,
  onClick: Function,
})

</script>

<template>
  <button class="thirdPartyConnexionButton" @click="onClick">
    <img :src="ico" alt="{{label}}" />
    <label>{{label}}</label>
  </button>
</template>

<style scoped>
  .thirdPartyConnexionButton {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    gap: 1rem;
  }

  img {
    height: 100%;
  }

  label {
    transform-origin: left center;
    transform: scaleX(0);
    display: none;
  }

  .thirdPartyConnexionButton:hover label {
    transform: scaleX(1);
    display: block;
  }
</style>