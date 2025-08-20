<script setup lang="ts">
import Button from '../Components/Button.vue';

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
    <Button @click="handleSearch" :disabled="disabled">
      <svg :style="{stroke: 'var(--primary-color-text)', transform: 'scale(1.5)'}" xmlns="http://www.w3.org/2000/svg"
           width="16" height="16" viewBox="0 0 24 24" fill="none" stroke-width="3" stroke-linecap="round"
           stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"></circle>
        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
      </svg>
    </Button>
  </div>
</template>

<style scoped>
.search-bar {
  display: flex;
  flex-direction: row;
  align-items: stretch;
  padding-left: var(--xs-spacing);
  max-width: 400px;
  max-height: 55px;
  border: 3px solid var(--primary-color);
   /*We add 3px to the border radius because the border is 3px wide.
   If we didn t do that there would an ugly space between .search-button
   and the border of .search-bar*/
  border-radius: calc(var(--m-border-radius) + 3px);
  flex-grow: 1;
}

.search-bar input {
  flex-grow: 1;
  border: none;
  background: transparent;
  outline: none;
  padding: var(--s-spacing) var(--s-spacing);
  width: 100%;
  font-size: var(--m-font-size);
}

.search-bar input::placeholder {
  color: #a0a0a0;
}
</style> 