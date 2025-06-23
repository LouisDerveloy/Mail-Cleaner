<template>
  <span class="tiptool-wrapper">
    <span class="tiptool-icon">&#9432;</span>
    <span class="tiptool-text">{{ props.text }}</span>
  </span>
</template>

<script setup lang="ts">
const props = defineProps<{ text: string }>()
</script>

<style scoped>
.tiptool-wrapper {
  position: relative;
  display: inline-block;
}
.tiptool-icon {
  cursor: pointer;
  color: #888;
  font-size: 1rem;
  margin-left: 0.25rem;
}
.tiptool-text {
  visibility: hidden;
  width: max-content;
  background: #333;
  color: #fff;
  text-align: left;
  border-radius: 4px;
  padding: 0.5em 1em;
  position: absolute;
  z-index: 10;
  left: 120%;
  top: 50%;
  transform: translateY(-50%);
  opacity: 0;
  transition: opacity 0.2s;
  white-space: pre-line;
}
.tiptool-wrapper:hover .tiptool-text {
  visibility: visible;
  opacity: 1;
}
</style> 