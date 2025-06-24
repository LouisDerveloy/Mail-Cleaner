<template>
  <div class="modal-overlay" v-if="searchStore.advanceSearch">
    <div class="modal-content">
      <header>
        <h1>Advanced search</h1>
        <button class="close-btn" @click="closeModal">×</button>
      </header>
      <form @submit.prevent="onSearch">
        <SearchField v-model="searchStore.text" label="Text" tip="Search for any text in the email." />
        <SearchField v-model="searchStore.body" label="Body" tip="Search within the email body." />
        <SearchField v-model="searchStore.before" label="Before" tip="Find emails before a specific date (dd-Mon-yyyy)." :is-date="true" />
        <SearchField v-model="searchStore.cc" label="CC" tip="Search by CC recipients." />
        <SearchField v-model="searchStore.from" label="From" tip="Search by sender." />
        <SearchField v-model="searchStore.new_" label="New" tip="Only show new emails." />
        <SearchField v-model="searchStore.since" label="Since" tip="Find emails since a specific date (dd-Mon-yyyy)." :is-date="true" />
        <SearchField v-model="searchStore.subject" label="Subject" tip="Search by subject line." />
        <SearchField v-model="searchStore.to" label="To" tip="Search by recipient." />
        <SearchField v-model="searchStore.unseen" label="Unseen" tip="Only show unseen emails." />
        <SearchField v-model="searchStore.seen" label="Seen" tip="Only show seen emails." />
        <button class="search-btn" type="submit">Search</button>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useSearchStore } from '../stores/search';
import SearchField from './SearchField.vue';

const props = defineProps<{ onSearch: () => void }>();

const searchStore = useSearchStore();

function onSearch() {
	searchStore.advanceSearch = false
  props.onSearch()
}

function closeModal() {
    searchStore.advanceSearch = false
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0; left: 0; right: 0; bottom: 0;
  background: rgba(0,0,0,0.6);
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;

  padding: 1rem;
}
.modal-content {
  background: #f3f3f3;
  width: 100vw;
  height: 100vh;
  max-width: 900px;
  max-height: 90vh;
  overflow-y: auto;
  position: relative;
  padding: 2rem;
  box-sizing: border-box;
  border-radius: 8px;

  display: flex;
  flex-direction: column;
}

.modal-content header {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;

    position: sticky;
    top: -2rem;
    background-color: #f3f3f3;

    z-index: 200;
}

.modal-content form {
	display: flex;
	flex-direction: row;
	flex-wrap: wrap;
	justify-content: space-between;
}
.close-btn {
  padding: 0;
  margin: 0;
  background: none;
  border: none;
  font-size: 4rem;
  cursor: pointer;
  align-self: flex-end;
}
.search-btn {
  width: 100%;
  padding: 1rem;
  background: #42b983;
  color: #fff;
  border: none;
  border-radius: 4px;
  font-size: 1.2rem;
  cursor: pointer;
  margin-top: 2rem;

  position: sticky;
  bottom: 0;

  box-shadow: -1px -1px 64px -12px rgba(10,10,10,0.57);
  -webkit-box-shadow: -1px -1px 64px -12px rgba(10,10,10,0.57);
  -moz-box-shadow: -1px -1px 64px -12px rgba(10,10,10,0.57);
}
</style>