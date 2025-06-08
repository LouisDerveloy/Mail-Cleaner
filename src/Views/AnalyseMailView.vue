<script setup lang="ts">
import { ref, shallowRef, computed } from "vue";
import { invoke, Channel } from "@tauri-apps/api/core";
import { confirm } from "@tauri-apps/plugin-dialog";
import type { Ref } from "vue";
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'
import { RecycleScroller } from 'vue-virtual-scroller'

interface Sender {
  id: number;
  name: string | null;
  email: string;
}

interface DisplaySender extends Sender {
  selected: boolean;
}

let senders: Ref<DisplaySender[]> = shallowRef([])
let analysing = ref(false)

const selectedCount = computed(() => {
  return senders.value.filter(s => s.selected).length
})

function start_analyse() {
  analysing.value = true
  senders.value = []
  const channel = new Channel()

  channel.onmessage = (response: unknown) => {
    const _senders = response as Sender[]
    const displaySenders: DisplaySender[] = _senders.map(s => ({ ...s, selected: false }))
    senders.value = [...senders.value, ...displaySenders]
  }

  invoke("get_list", {"retChannel": channel, "query": "BODY unsubscribe"}).then(() => {
    analysing.value = false
  })
}

function test_button() {
  invoke("test").then(value => {
    console.dir(value)
  })
}

function onSelect(id: number) {
  console.log("selected: ", id)
  senders.value = senders.value.map(s => s.id === id ? { ...s, selected: true } : s)
}

function onUnselect(id: number) {
  console.log("unselected: ", id)
  senders.value = senders.value.map(s => s.id === id ? { ...s, selected: false } : s)
}

async function selectAll() {
  const result = await confirm("Are you sure you want to select all items?", {
    title: "Select All",
    kind: "info",
  });
  if (result) {
    senders.value = senders.value.map(s => ({ ...s, selected: true }));
  }
}

async function unselectAll() {
  const result = await confirm("Are you sure you want to unselect all items?", {
    title: "Unselect All",
    kind: "warning",
  });
  if (result) {
    senders.value = senders.value.map(s => ({ ...s, selected: false }));
  }
}

</script>
<template>
  <h1>This is the Mail analyse View</h1>
  <section class="controls">
    <button @click="test_button">Test Button</button>
    <button @click="start_analyse" :disabled="analysing">Analyse</button>
    <button @click="selectAll">Select All</button>
    <button @click="unselectAll">Unselect All</button>
    <span>Selected: {{ selectedCount }}</span>
  </section>
  <RecycleScroller
      class="SenderList"
      :items="senders"
      :item-size="32"
      key-field="id"
      v-slot="{ item }"
  >
    <div class="sender-item">
      <button v-if="!item.selected" class="select-button" @click="onSelect(item.id)">Select</button>
      <button v-else class="unselect-button" @click="onUnselect(item.id)">Unselect</button>
      <span>{{ item.email }}</span>
    </div>
  </RecycleScroller>
</template>

<style scoped>

.controls {
  display: flex;
  flex-direction: row;
  gap: 1rem;
}

.SenderList {
  overflow-y: auto;
}

.sender-item {
  height: 32px;
  padding: 0 12px;
  display: flex;
  align-items: center;
  gap: 1rem;
}

.unselect-button {
  background: #f55151;
}

.select-button {
  background: #51f560;
}

.sender-item:hover {
  background-color: #d9d9d9;
}
</style>