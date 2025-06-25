<script setup lang="ts">
const model = defineModel<string>()

defineProps<{
  disabled?: boolean
}>()

const emit = defineEmits<{
  (e: 'search'): void
}>()

function handleSearch() {
  emit('search')
}
</script>

<template>
  <div class="search-bar">
    <input 
      type="text" 
      name="query" 
      id="query" 
      v-model="model" 
      placeholder="Search..." 
      @keyup.enter="handleSearch" 
      :disabled="disabled"
    >
    <button @click="handleSearch" :disabled="disabled" class="search-button">
      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"></circle>
        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
      </svg>
    </button>
  </div>
</template>

<style scoped>
.search-bar {
  display: flex;
  flex-direction: row;
  align-items: center;
  background-color: #dfdfdf;
  border-radius: 25px;
  padding-left: 8px;
  max-width: 400px;
  max-height: 36px;
  border: 2px solid #cfcfcf;
  flex-grow: 1;
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
</style> 