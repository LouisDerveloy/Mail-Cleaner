<!--
  - Copyright (c) 2025. Louis DERVELOY (https://github.com/LouisDerveloy/)
  - Licensed under the PolyForm Noncommercial License 1.0.0
  - SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
  - See the LICENSE file in the project root for details.
  -->

<script setup lang="ts">
import {openUrl} from '@tauri-apps/plugin-opener';
import copy_paste from '../assets/images/copy_paste.png';
import {writeText} from '@tauri-apps/plugin-clipboard-manager';
import gsap from 'gsap';
import {onMounted, onBeforeUnmount, ref} from "vue";

interface Props {
  href: string
  copybutton?: boolean
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

let copyBtn = ref<HTMLButtonElement | null>(null);
let ctx: gsap.Context | null = ref(null);
onMounted(() => {
  ctx = gsap.context(() => {
    if (copyBtn.value === null) return;

    copyBtn.value.addEventListener('mousedown', () => gsap.to(copyBtn.value, {scale: 0.8, duration: 0.1, ease: "power1.out"}));
    const releaseAnimation = () => gsap.to(copyBtn.value, {scale: 1, duration: 0.1, ease: "power1.out"});
    copyBtn.value.addEventListener('mouseup', releaseAnimation);
    copyBtn.value.addEventListener('mouseleave', releaseAnimation);

      }
  );
});
onBeforeUnmount(() => ctx?.revert())
</script>
<template>
  <span class="external_link">
    <a
        :href="href"
        rel="noopener"
        @click="onClick"
        @auxclick="onAuxClick"
    >
      <slot/>
    </a>
    <button v-if="props.copybutton" ref="copyBtn" @click="writeText(props.href);"><img :src="copy_paste" alt="Copy to clipboard"/></button>
  </span>
</template>
<style scoped>
.external_link {
  display: flex;
  flex-direction: row;
  align-items: center;
  flex-wrap: nowrap;
  gap: var(--s-spacing);
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
  border: none;
  outline: none;
}

.external_link button:focus {
  outline: none;
  border: none;
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
