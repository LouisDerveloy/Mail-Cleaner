<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener';
import copy_paste from '../assets/images/copy_paste.png';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

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
  <div class="external_link">
    <a
        :href="href"
        rel="noopener"
        @click="onClick"
        @auxclick="onAuxClick"
    >
      <slot  />
    </a>
    <button @click="writeText(props.href);"><img :src="copy_paste" alt="Copy to clipboard" /></button>
  </div>
</template>
<style scoped>
.external_link {
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
  gap: 1rem;
}

.external_link a {
  background: none;
  color: #2a6ae9;
  text-decoration: underline;
  cursor: pointer;
  font-style: italic;
  overflow: hidden;
  text-overflow: ellipsis;
  max-lines: 1;
}

.external_link button {
  background: transparent;
  margin: 0;
  padding: 0;
  position: relative;
}

.external_link button::before {
  content: 'Copy to clipboard';

  display: none;
  padding: .5rem;
  border-radius: 10px;

  position: absolute;
  top: 0;
  right: 0;
  transform: translate(100%, -100%);

  background-color: hsl(0, 0%, 95%);
  outline: black;
}

.external_link button:hover::before {
  display: block;
}

.external_link button img {
  width: 32px;
  aspect-ratio: 1/1;
}
</style>
