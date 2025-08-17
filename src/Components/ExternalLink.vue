<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener';

interface Props {
  href: string
}
const props = defineProps<Props>();

function isHttpLike(url: string) {
  return url.startsWith('https://') || url.startsWith('http://') || url.startsWith('mailto:');
}

async function openExternal() {
  // Optionnel: simple validation
  if (!isHttpLike(props.href)) return;
  await openUrl(props.href); // ouvre avec l’app par défaut (navigateur, client mail, etc.)
}

function onClick(e: MouseEvent) {
  e.preventDefault();
  openExternal();
}

function onAuxClick(e: MouseEvent) {
  // Middle click
  if (e.button !== 1) return;
  e.preventDefault();
  openExternal();
}
</script>
<template>
  <!-- href réel pour l’accessibilité et copier l’adresse -->
  <a
      :href="href"
      rel="noopener"
      @click="onClick"
      @auxclick="onAuxClick"
  >
    <slot  />
  </a>
</template>
<style scoped>
a {
  background: none;
  color: #2a6ae9;
  text-decoration: underline;
  cursor: pointer;
  font-style: italic;
}
</style>
