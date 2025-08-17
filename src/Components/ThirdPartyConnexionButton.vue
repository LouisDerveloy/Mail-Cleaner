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
    background-color: hsl(0, 0%, 95%);
    box-shadow: 0 0 20px 5px hsla(0, 0%, 0%, 0.1);
    border-radius: 20%;
  }

  label {
    transform-origin: left center;
    transform: scaleX(0);
    opacity: 0;
    display: none;
  }

  @keyframes fade-in-left {
    from {
      display: block;
      opacity: 0;
      transform: translateX(-50px);
      filter: blur(10px);
    }

    to {
      display: block;
      opacity: 1;
      transform: translateX(0);
      filter: blur(0px);
    }
  }

  .thirdPartyConnexionButton:hover label {
    animation: fade-in-left-fd637c1a .4s forwards cubic-bezier(0, 0, 0.2, 0.98);
    display: block;
    opacity: 1;
  }
</style>