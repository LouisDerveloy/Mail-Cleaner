<script setup lang="ts">
import {onMounted, ref} from "vue";
import { invoke, Channel } from "@tauri-apps/api/core";
import type { Ref } from "vue";

const ret_channel = new Channel()
let senders: Ref<Array<{id: number; name: string; email: string}>> = ref([])

onMounted(() => {
  ret_channel.onmessage = (sender: any) => {

    console.log("Pushed a sender email")
    senders.value.push(sender)

  }
})

function start_analyse() {
  invoke("get_list", {"retChannel": ret_channel, "query": "BODY unsubscribe"})
}

function test_button() {
  invoke("test").then(value => {
    console.dir(value)
  })
}

</script>
<template>
<h1>This is the Mail analyse View</h1>
  <button @click="test_button">Test Button</button>
  <button @click="start_analyse">Analyse</button>
  <div class="SenderList">
    <ul v-for="sender in senders" :key="sender.id" ref="ul_senders_list">
      <li>Added {{sender.email}}</li>
    </ul>
  </div>
</template>

<style scoped>
  .SenderList {
    height: 50vh;
    overflow: scroll;
  }

  .SenderList li {
    background-color: white;
    width: 100%;
    padding: .2rem;
    list-style-type: none;
  }

  .SenderList li:hover {
    background-color: #d9d9d9;
  }
</style>