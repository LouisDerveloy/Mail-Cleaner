<script setup lang="ts">
import {onMounted, shallowRef} from "vue";
import { invoke, Channel } from "@tauri-apps/api/core";
import type { Ref } from "vue";

interface Sender {
  id: number;
  name: string | null;
  email: string;
}

let senders: Ref<Sender[]> = shallowRef([])

function start_analyse() {
  senders.value = []
  const channel = new Channel()
  
  channel.onmessage = (response: unknown) => {
    console.log("channel response", response)
    const _senders = response as Sender[]
    senders.value = [...senders.value, ..._senders]
  }

  invoke("get_list", {"retChannel": channel, "query": "BODY unsubscribe"})
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
    <ul>
      <li v-for="(sender, index) in senders" :key="index">
        Added {{sender.email}}
      </li>
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