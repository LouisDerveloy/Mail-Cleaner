<script setup lang="ts">
const props = withDefaults(defineProps<{
  label?: string,
  ico?: string;
  alt?: string;
  onClick?: Function;
}>(), { label: '', ico: '', alt: '' });
</script>

<template>
  <button class="TabItem" @click="onClick">
    <img v-if="props.ico !== ''" :src="props.ico" :alt="props.alt" />
    <span v-if="label !== ''">{{props.label}}</span>
  </button>
</template>

<style scoped>
.TabItem {
  display:flex;
  flex-direction: row;
  justify-content: stretch;
  align-items: center;
  gap: var(--xs-spacing);
  background: none;
  outline: none;
  border: none;
  z-index: 3;
  padding: var(--s-spacing);
  color: var(--primary-color-text);
  font-size: var(--m-font-size);
  overflow: hidden;
  text-overflow: ellipsis;
}

.TabItem span {
  mix-blend-mode: difference;
}

.TabItem * {
  max-height: 32px;
}
</style>