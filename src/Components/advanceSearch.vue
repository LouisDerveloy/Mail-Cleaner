<template>
  <div class="modal-overlay" v-if="searchStore.advanceSearch">
    <div class="modal-content">
      <header>
        <h1>Advanced search</h1>
        <button class="close-btn" @click="closeModal">×</button>
      </header>
      <form @submit.prevent="onSearch">
        <div class="form-group" v-for="field in fields" :key="field.key">
          <label :for="field.key">{{ field.label }}
            <TipTool :text="field.tip" />
          </label>
          <template v-if="field.type === 'text'">
            <input v-model="(searchStore[field.key] as any)" :id="field.key" type="text" />
          </template>
          <template v-else-if="field.type === 'checkbox'">
            <input v-model="(searchStore[field.key] as any)" :id="field.key" type="checkbox" />
          </template>
        </div>
        <button class="search-btn" type="submit">Search</button>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useSearchStore } from '../stores/search';
import TipTool from './TipTool.vue';
const props = defineProps<{ onSearch: () => void }>();

const searchStore = useSearchStore();

interface Field {
  key: keyof typeof searchStore;
  label: string;
  type: 'text' | 'checkbox';
  tip: string;
}

const fields: Field[] = [
  { key: 'text', label: 'Text', type: 'text', tip: 'Search for any text in the email.' },
  { key: 'body', label: 'Body', type: 'text', tip: 'Search within the email body.' },
  { key: 'before', label: 'Before', type: 'text', tip: 'Find emails before a specific date (YYYY-MM-DD).' },
  { key: 'cc', label: 'CC', type: 'text', tip: 'Search by CC recipients.' },
  { key: 'from', label: 'From', type: 'text', tip: 'Search by sender.' },
  { key: 'new_', label: 'New', type: 'checkbox', tip: 'Only show new emails.' },
  { key: 'since', label: 'Since', type: 'text', tip: 'Find emails since a specific date (YYYY-MM-DD).' },
  { key: 'subject', label: 'Subject', type: 'text', tip: 'Search by subject line.' },
  { key: 'to', label: 'To', type: 'text', tip: 'Search by recipient.' },
  { key: 'unseen', label: 'Unseen', type: 'checkbox', tip: 'Only show unseen emails.' },
  { key: 'seen', label: 'Seen', type: 'checkbox', tip: 'Only show seen emails.' },
];

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
.form-group {
	width: 100%;
	max-width: 400px;
    margin-bottom: 1.5rem;
    display: flex;
    flex-direction: column;
}
label {
  font-weight: bold;
  margin-bottom: 0.5rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}
input[type="text"] {
  padding: 0.5rem;
  border: 1px solid #ccc;
  border-radius: 4px;
}
input[type="checkbox"] {
  margin-left: 0.5rem;
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