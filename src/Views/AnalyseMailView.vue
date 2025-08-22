<!--
  - Copyright (c) 2025. Louis DERVELOY (https://github.com/LouisDerveloy/)
  - Licensed under the PolyForm Noncommercial License 1.0.0
  - SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
  - See the LICENSE file in the project root for details.
  -->

<script lang="ts" setup>
import type {Ref} from "vue";
import {computed, onBeforeUnmount, onMounted, ref, shallowRef} from "vue";
import {Channel, invoke} from "@tauri-apps/api/core";
import {confirm, message} from "@tauri-apps/plugin-dialog";
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'
import {RecycleScroller} from 'vue-virtual-scroller'
import {handleError} from "../lib/error";
import noResultsImage from "/src/assets/images/no-results.png";
import searchingImage from "/src/assets/images/searching.png";
import {useSearchStore} from "../stores/search.ts";
import AdvanceSearch from "../Components/advanceSearch.vue"
import SearchBar from "../Components/SearchBar.vue";
import Button from "../Components/Button.vue";
import {useRouter} from "vue-router";
import gsap from "gsap";

const router = useRouter();

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
let progress = ref({current: 0, total: 0});

const selectedCount = computed(() => {
  return senders.value.filter(s => s.selected).length
})

function start_search() {
  isProcessing.value = true
  searching.value = true
  senders.value = []
  progress.value = {current: 0, total: 0};
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

  invoke<Sender[]>("get_list", {retChannel: channel, query: search}).then((result) => {
    const displaySenders: DisplaySender[] = result.map(s => ({...s, selected: false}))
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
  senders.value = senders.value.map(s => s.id === id ? {...s, selected: true} : s)
}

function onUnselect(id: number) {
  console.log("unselected: ", id)
  senders.value = senders.value.map(s => s.id === id ? {...s, selected: false} : s)
}

async function selectAll() {
  const result = await confirm("Are you sure you want to select all items?", {
    title: "Select All",
    kind: "info",
  });
  if (result) {
    senders.value = senders.value.map(s => ({...s, selected: true}));
  }
}

async function unselectAll() {
  const result = await confirm("Are you sure you want to unselect all items?", {
    title: "Unselect All",
    kind: "warning",
  });
  if (result) {
    senders.value = senders.value.map(s => ({...s, selected: false}));
  }
}

async function deleteSelected() {
  if (selectedCount.value === 0) {
    await message("There are no emails selected to delete.", {title: "Delete Senders", kind: "info"});
    return;
  }

  const confirmed = await confirm(`Are you sure you want to delete all emails coming from ${selectedCount.value} senders? Please be aware that all the emails from each sender selected will be deleted not only the ones matching the search query.`, {
    title: "Confirm Deletion",
    kind: "warning",
  });

  if (confirmed) {
    const idsToDelete = senders.value.filter(s => s.selected).map(s => s.id);

    isProcessing.value = true;
    progress.value = {current: 0, total: 0};

    const channel = new Channel();

    channel.onmessage = (response: unknown) => {
      progress.value = response as Progress;
    };

    try {
      await invoke("delete_senders", {senderIds: idsToDelete, retChannel: channel}).catch(handleError);
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

let ctx: gsap.Context | null = null
onMounted(() => ctx = gsap.context(() => {
  gsap.fromTo('.fade-in', {
    opacity: 0,
    y: "-50",
  }, {
    opacity: 1,
    y: 0,
    duration: .8,
    stagger: .2,
    ease: "bounce.out"
  })
}));
onBeforeUnmount(() => ctx?.revert())

</script>
<template>
  <section class="analyse-view">
    <section class="controls fade-in">
      <div class="search-container">
        <SearchBar v-model="searchStore.text" :disabled="isProcessing" @search="start_search"/>
        <Button :disabled="isProcessing" @click="toggleAdvanceSearch">Advance Search</Button>
      </div>
      <Button @click="router.push('user/connexion')">Sign in</Button>
      <Button :disabled="isProcessing || senders.length === 0" @click="clearList">Clear List</Button>
      <Button @click="selectAll">Select All</Button>
      <Button @click="unselectAll">Unselect All</Button>
      <Button :disabled="selectedCount === 0" variant="red" @click="deleteSelected">Delete</Button>
    </section>
    <section class="table">
      <div class="header">
        <span class="fade-in">Email (Selected: {{ selectedCount }})</span>
        <span class="fade-in">Occurrence</span>
      </div>
      <RecycleScroller
          v-if="senders.length > 0"
          v-slot="{ item }"
          :item-size="50"
          :items="senders"
          class="SenderList"
          key-field="id"
      >
        <div class="sender-item">
          <Button v-if="!item.selected" size="small" @click="onSelect(item.id)">Select</Button>
          <Button v-else size="small" variant="red" @click="onUnselect(item.id)">Unselect</Button>
          <span>{{ item.email }}</span>
          <span class="occurrence">{{ item.occurrence }}</span>
        </div>
      </RecycleScroller>
      <div v-else-if="!searching" class="info">
        <img :src="noResultsImage" alt="No results" class="cat-image fade-in">
        <span class="fade-in">I can't find any mails meow. Maybe you should start a search.</span>
        <SearchBar v-model="searchStore.text" :disabled="isProcessing" class="fade-in" @search="start_search"/>
      </div>
      <div v-else class="info">
        <img :src="searchingImage" alt="Searching..." class="cat-image fade-in">
        <span class="fade-in">I'm searching for your emails... I hope i will get some treats for that.</span>
      </div>
    </section>
    <div v-if="isProcessing || progress.total > 0" class="progress-container">
      <progress :max="progress.total" :value="progress.current"/>
      <span v-if="progress.current === 0 && isProcessing">Loading...</span>
      <span v-else>{{ progress.current }} / {{ progress.total }}</span>
    </div>
  </section>
  <AdvanceSearch :onSearch="start_search"/>
</template>

<style scoped>

.analyse-view {
  height: 100%;
  padding: var(--s-spacing);
  display: flex;
  flex-direction: column;
  gap: var(--m-spacing);
}

.controls {
  display: flex;
  flex-wrap: wrap;
  flex-direction: row;
  justify-content: end;
  align-items: center;
  column-gap: var(--s-spacing);
  row-gap: var(--s-spacing);
}

.table {
  display: flex;
  flex-direction: column;
  height: 50%;

  flex-grow: 1;

  border: 2px solid var(--secondary-color);
  border-radius: var(--l-border-radius);
}

.table .info {
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--s-spacing);
}

.table .info img {
  width: 300px;
  height: 300px;
}

.table .info span {
  display: block;
  font-size: var(--l-font-size);
}

.table .header {
  padding: var(--s-spacing) var(--m-spacing);
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  font-weight: 600;
  text-transform: uppercase;
}

.SenderList {
  overflow-y: scroll;
  scroll-behavior: smooth;
}

.sender-item {
  height: 50px;
  padding: 0 var(--s-spacing);
  display: flex;
  align-items: center;
  gap: var(--s-spacing);
  border-bottom: 2px solid #cfcfcf;
}

.occurrence {
  margin-left: auto;
}

.sender-item:hover {
  background-color: #d9d9d9;
}

.search-container {
  display: flex;
  flex-direction: row;
  gap: var(--s-spacing);
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
  gap: var(--xs-spacing);
  height: 25px;
  border-radius: 12px;
  overflow: hidden;
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