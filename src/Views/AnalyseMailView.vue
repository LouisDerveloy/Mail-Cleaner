<script setup lang="ts">
import { ref, shallowRef, computed } from "vue";
import { invoke, Channel } from "@tauri-apps/api/core";
import { confirm, message } from "@tauri-apps/plugin-dialog";
import type { Ref } from "vue";
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'
import { RecycleScroller } from 'vue-virtual-scroller'
import { handleError } from "../lib/error";
import noResultsImage from "/src/assets/images/no-results.png";
import searchingImage from "/src/assets/images/searching.png";
import { useSearchStore } from "../stores/search.ts";
import AdvanceSearch from "../Components/advanceSearch.vue"
import SearchBar from "../Components/SearchBar.vue";

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

const searchStore = useSearchStore()

let senders: Ref<DisplaySender[]> = shallowRef([])
let isProcessing = ref(false)
let searching = ref(false)
let progress = ref({ current: 0, total: 0 });

const selectedCount = computed(() => {
  return senders.value.filter(s => s.selected).length
})

function start_search() {
  isProcessing.value = true
  searching.value = true
  senders.value = []
  progress.value = { current: 0, total: 0 };
  const channel = new Channel()

  channel.onmessage = (response: unknown) => {
    progress.value = response as Progress;
  }

  // Gather all searchStore state into a Search object
  const search: Record<string, any> = {
    text: searchStore.text || undefined,
    body: searchStore.body || undefined,
    before: searchStore.before || undefined,
    cc: searchStore.cc || undefined,
    from: searchStore.from || undefined,
    new_: searchStore.new_,
    since: searchStore.since || undefined,
    subject: searchStore.subject || undefined,
    to: searchStore.to || undefined,
    unseen: searchStore.unseen,
    seen: searchStore.seen,
  };

  // Remove empty string fields (convert to undefined)
  Object.keys(search).forEach(key => {
    if (typeof search[key] === 'string' && search[key].trim() === '') {
      search[key] = undefined;
    }
  });

  invoke<Sender[]>("get_list", { retChannel: channel, query: search }).then((result) => {
    const displaySenders: DisplaySender[] = result.map(s => ({ ...s, selected: false }))
    senders.value = displaySenders;
    isProcessing.value = false
    searching.value = false
  }).catch(async (err) => {
    await handleError(err);
    isProcessing.value = false;
    searching.value = false
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

  const confirmed = await confirm(`Are you sure you want to delete all emails coming from ${selectedCount.value} senders? Please be aware that all the emails from each sender selected will be deleted not only the ones matching the search query.`, {
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

function clearList() {
  // No need to clear in the backend, it will be cleared when the user starts a new search.
  // We just need to clear the UI.
  senders.value = [];
}

function toggleAdvanceSearch() {
  searchStore.advanceSearch = !searchStore.advanceSearch
}

</script>
<template>
  <section class="analyse-view">
    <section class="controls">
      <div class="search-container">
        <SearchBar v-model="searchStore.text" @search="start_search" :disabled="isProcessing" />
        <button @click="toggleAdvanceSearch" :disabled="isProcessing">Advance Search</button>
      </div>
      <RouterLink to="user/connexion">Connect</RouterLink>
      <button @click="clearList" :disabled="isProcessing || senders.length === 0">Clear List</button>
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
    <div class="info" v-else-if="!searching">
      <img :src="noResultsImage" alt="No results" class="cat-image">
      <span>I can't find any mails meow. Maybe you should start a search.</span>
      <SearchBar v-model="searchStore.text" @search="start_search" :disabled="isProcessing" />
    </div>
    <div class="info" v-else>
      <img :src="searchingImage" alt="Searching..." class="cat-image">
      <span>I'm searching for your emails... I hope i will get some treats for that.</span>
    </div>
    </section>
    <div class="progress-container" v-if="isProcessing || progress.total > 0">
      <progress :value="progress.current" :max="progress.total" />
      <span v-if="progress.current === 0 && isProcessing">Loading...</span>
      <span v-else>{{ progress.current }} / {{ progress.total }}</span>
    </div>
  </section>
  <AdvanceSearch :onSearch="start_search" />
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

.table .info {
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 1rem;
}

.table .info img {
  width: 300px;
  height: 300px;
}

.table .info span {
  display: block;
  font-size: 2rem;
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

.search-container {
  display: flex;
  flex-direction: row;
  gap: 1rem;
  margin-right: auto;
  align-items: center;
  flex-grow: 1;
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