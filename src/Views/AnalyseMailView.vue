<script setup lang="ts">
import { ref, shallowRef, computed } from "vue";
import { invoke, Channel } from "@tauri-apps/api/core";
import { confirm, message } from "@tauri-apps/plugin-dialog";
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
let searching = ref(false)
let query = ref("BODY unsubscribe")

const selectedCount = computed(() => {
  return senders.value.filter(s => s.selected).length
})

function start_search() {
  searching.value = true
  senders.value = []
  const channel = new Channel()

  channel.onmessage = (response: unknown) => {
    const _senders = response as Sender[]
    const displaySenders: DisplaySender[] = _senders.map(s => ({ ...s, selected: false }))
    senders.value = [...senders.value, ...displaySenders]
  }

  invoke("get_list", {"retChannel": channel, "query": query.value}).then(() => {
    searching.value = false
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

async function deleteSelected() {
  if (selectedCount.value === 0) {
    await message("There are no emails selected to delete.", { title: "Delete Senders", kind: "info" });
    return;
  }

  const confirmed = await confirm(`Are you sure you want to delete all emails coming from ${selectedCount.value} senders?`, {
    title: "Confirm Deletion",
    kind: "warning",
  });

  if (confirmed) {
    const idsToDelete = senders.value.filter(s => s.selected).map(s => s.id);
    
    await invoke("delete_senders", { senderIds: idsToDelete });
    senders.value = [];
  }
}

</script>
<template>
  <section class="analyse-view">
    <section class="controls">
    <button @click="test_button">Test Button</button>
    <input type="text" name="query" id="query" v-model="query">
    <button @click="start_search" :disabled="searching">Search</button>
    <button @click="selectAll">Select All</button>
    <button @click="unselectAll">Unselect All</button>
    <button @click="deleteSelected" :disabled="selectedCount === 0" class="delete-button">Delete Selected</button>
  </section>
  <h3>Selected: {{ selectedCount }}</h3>
  <div class="spacer"></div>
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
  </section>
</template>

<style scoped>

.analyse-view {
  height: 100%;
  padding: .25rem;
  display: flex;
  flex-direction: column;
  gap: .5rem;
}

.controls {
  display: flex;
  flex-wrap: wrap;
  flex-direction: row;
  align-items: center;
  column-gap: .5rem;
  row-gap: .25rem;
}

.spacer {
  height: 2px;
  width: 95%;
  margin: 0 auto;
  background-color: #cfcfcf;
}

.SenderList {
  overflow-y: scroll;
}

.sender-item {
  height: 32px;
  padding: 0 12px;
  display: flex;
  align-items: center;
  gap: 1rem;
}

.unselect-button, .delete-button {
  background: #f55151;
}

.select-button {
  background: #51f560;
}

.sender-item:hover {
  background-color: #d9d9d9;
}
</style>