<script setup lang="ts">
import { shallowRef } from "vue";
import { invoke, Channel } from "@tauri-apps/api/core";
import type { Ref } from "vue";
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'
import { RecycleScroller } from 'vue-virtual-scroller'

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
  <RecycleScroller
      class="SenderList"
      :items="senders"
      :item-size="32"
      key-field="id"
      v-slot="{ item }"
  >
    <div class="sender-item">
      Added {{ item.email }}
    </div>
  </RecycleScroller>
</template>

<style scoped>
.SenderList {
  height: 50vh;
  overflow-y: auto;
}

.sender-item {
  height: 32px;
  padding: 0 12px;
  display: flex;
  align-items: center;
}

.sender-item:hover {
  background-color: #d9d9d9;
}
</style>