<script setup lang="ts">
import { ref, shallowRef, computed } from "vue";
import { invoke, Channel } from "@tauri-apps/api/core";
import { confirm, message } from "@tauri-apps/plugin-dialog";
import type { Ref } from "vue";
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'
import { RecycleScroller } from 'vue-virtual-scroller'
import { handleError } from "../lib/error";

interface Sender {
  id: number;
  name: string | null;
  email: string;
  occurrence: number;
}

interface DisplaySender extends Sender {
  selected: boolean;
}

interface Progress {
  current: number,
  total: number
};

let senders: Ref<DisplaySender[]> = shallowRef([])
let isProcessing = ref(false)
let query = ref("BODY unsubscribe")
let progress = ref({ current: 0, total: 0 });

const selectedCount = computed(() => {
  return senders.value.filter(s => s.selected).length
})

function start_search() {
  isProcessing.value = true
  senders.value = []
  progress.value = { current: 0, total: 0 };
  const channel = new Channel()

  channel.onmessage = (response: unknown) => {
    progress.value = response as Progress;
  }

  invoke<Sender[]>("get_list", {"retChannel": channel, "query": query.value}).then((result) => {
    const displaySenders: DisplaySender[] = result.map(s => ({ ...s, selected: false }))
    senders.value = displaySenders;
    isProcessing.value = false
  }).catch(async (err) => {
    await handleError(err);
    isProcessing.value = false;
  });
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
    
    isProcessing.value = true;
    progress.value = { current: 0, total: 0 };
    const channel = new Channel();

    channel.onmessage = (response: unknown) => {
      progress.value = response as Progress;
    };

    try {
      await invoke("delete_senders", { senderIds: idsToDelete, retChannel: channel });
      senders.value = senders.value.filter(s => !s.selected);
    } catch (err) {
      await handleError(err);
    } finally {
      isProcessing.value = false;
    }
  }
}

</script>
<template>
  <section class="analyse-view">
    <section class="controls">
      <div class="search-bar">
        <input type="text" name="query" id="query" v-model="query" placeholder="Search..." @keyup.enter="start_search" :disabled="isProcessing">
        <button @click="start_search" :disabled="isProcessing" class="search-button">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="11" cy="11" r="8"></circle>
            <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
          </svg>
        </button>
      </div>
      <RouterLink to="user/connexion">Connect</RouterLink>
      <button @click="selectAll">Select All</button>
      <button @click="unselectAll">Unselect All</button>
      <button @click="deleteSelected" :disabled="selectedCount === 0" class="delete-button">Delete Selected</button>
    </section>
    <section class="table">
      <div class="header">
        <span>Email (Selected: {{ selectedCount }})</span>
        <span>Occurrence</span>
      </div>
      <RecycleScroller
        class="SenderList"
        :items="senders"
        :item-size="32"
        key-field="id"
        v-slot="{ item }"
        v-if="senders.length > 0"
    >
      <div class="sender-item">
        <button v-if="!item.selected" class="select-button" @click="onSelect(item.id)">Select</button>
        <button v-else class="unselect-button" @click="onUnselect(item.id)">Unselect</button>
        <span>{{ item.email }}</span>
        <span class="occurrence">{{ item.occurrence }}</span>
      </div>
    </RecycleScroller>
    <div class="no-results" v-else>
      <span>You have no emails to delete. Please search for emails to delete.</span>
    </div>
    </section>
    <div class="progress-container" v-if="isProcessing || progress.total > 0">
      <progress :value="progress.current" :max="progress.total" />
      <span>{{ progress.current }} / {{ progress.total }}</span>
    </div>
  </section>
</template>

<style scoped>

.analyse-view {
  height: 100%;
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;

  margin-top: .5rem;
}

.controls {
  display: flex;
  flex-wrap: wrap;
  flex-direction: row;
  justify-content: end;
  align-items: center;
  column-gap: .5rem;
  row-gap: .25rem;
}

.table {
  display: flex;
  flex-direction: column;
  gap: .25rem;
  height: 100%;
}

.table .no-results {
  height: 100%;
  width: 100%;
  display: grid;
  place-items: center;
}

.table .no-results span {
  display: block;
}

.table .header {
  padding: .7rem 0;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid #cfcfcf;
  font-weight: 600;
  text-transform: uppercase;
}

.SenderList {
  overflow-y: scroll;
  border-radius: 5px;
}

.sender-item {
  height: 32px;
  padding: 0 1rem;
  display: flex;
  align-items: center;
  gap: 1rem;
}

.occurrence {
  margin-left: auto;
  color: #555;
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

.search-bar {
  margin-right: auto;
  display: flex;
  flex-direction: row;
  align-items: center;
  background-color: #dfdfdf;
  border-radius: 25px;
  padding-left: 8px;
  max-width: 600px;
  border: 2px solid #cfcfcf;
}

.search-bar input {
  flex-grow: 1;
  border: none;
  background: transparent;
  outline: none;
  padding: 0 .5rem;
  width: 100%;
}

.search-bar input::placeholder {
  color: #a0a0a0;
}

.search-bar .search-button {
  border: none;
  background: white;
  border-radius: 50%;
  width: 32px;
  height: 32px;
  min-width: 32px;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  padding: 0;
  margin: 0;
}

.search-bar .search-button:disabled {
  cursor: not-allowed;
  opacity: 0.7;
}

.search-bar .search-button svg {
  stroke: #f55151;
}

.progress-container {
  margin-top: auto;
  position: relative;
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: .25rem;
  height: 25px;
  border-radius: 12px;
  overflow: hidden;
  border: 2px solid #cfcfcf;
}

.progress-container progress {
  width: 100%;
  height: 100%;
  border: none;
}

.progress-container progress::-webkit-progress-bar {
  background-color: #ffffff;
}

.progress-container progress::-webkit-progress-value {
  background-color: #51f560;
}

.progress-container span {
  font-weight: 800;
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
}

</style>