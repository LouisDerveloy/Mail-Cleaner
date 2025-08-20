<script setup lang="ts">
import Button from "../Components/Button.vue";
import TabContainer from "../Components/Tab/TabContainer.vue";
import TabItem from "../Components/Tab/TabItem.vue";
import Gmailico from "../assets/images/gmail.svg";
import Vueico from "../assets/vue.svg";
</script>

<template>
  <section class="center">
    <div class="wrapper">
      <TabContainer style="max-width: 600px">
        <TabItem label="Home"/>
        <TabItem label="About"/>
        <TabItem label="Contact"/>
        <TabItem label="Sign in with Google" :ico="Gmailico"/>
        <TabItem label="Showcases"/>
        <TabItem label="Sign in with Microsoft"/>
        <TabItem :ico="Vueico"/>
      </TabContainer>
      <section>
        <h1>lorem ipsum dolor sit amet consectetur</h1>
        <h2>elit cillum feugait eu placerat aliquyam</h2>
        <h3>gubergren aliquid ut aliquam doming justo</h3>
      </section>
      <Button :icon="Vueico" size="small" @click="console.log('button A')">I'm primary and small</Button>
      <Button :icon="Gmailico" variant="red" expand @click="console.log('button Test')">I'm medium red and expanded</Button>
      <Button :icon="Gmailico" variant="secondary" @click="console.log('Signed in with google')" size="large">I'm secondary and large</Button>
      <Button :icon="Gmailico" @click="console.log('Disabled button')" disabled>I'm disabled</Button>
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