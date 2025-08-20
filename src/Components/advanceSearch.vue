<script lang="ts" setup>
import {useSearchStore} from '../stores/search';
import Input from './Input.vue';
import CrossIcon from '../assets/images/cross.svg';
import Button from '../components/Button.vue';

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
<template>
  <div ref="advanceStore" v-if="searchStore.advanceSearch" class="modal-overlay">
    <div class="modal-content">
      <header>
        <h1>Advanced search</h1>
        <button class="close-btn" @click="closeModal"><img :src="CrossIcon"
                                                           :style="{aspectRatio: '1/1', height: '32px'}"
                                                           alt="Cross icon to close the modal.">
        </button>
      </header>
      <form @submit.prevent="onSearch">
        <Input type="text" v-model="searchStore.text" label="Text" tip="Search for any text in the email."/>
        <Input type="text"  v-model="searchStore.body" label="Body" tip="Search within the email body."/>
        <Input type="text"  v-model="searchStore.before" :is-date="true" label="Before"
                     tip="Find emails before a specific date (dd-Mon-yyyy)."/>
        <Input type="text"  v-model="searchStore.cc" label="CC" tip="Search by CC recipients."/>
        <Input type="date"  v-model="searchStore.from" label="From" tip="Search by sender."/>
        <Input type="date"  v-model="searchStore.since" :is-date="true" label="Since"
                     tip="Find emails since a specific date (dd-Mon-yyyy)."/>
        <Input type="text"  v-model="searchStore.subject" label="Subject" tip="Search by subject line."/>
        <Input type="text"  v-model="searchStore.to" label="To" tip="Search by recipient."/>
        <Input type="checkbox"  v-model="searchStore.new_" label="New" tip="Only show new emails."/>
        <Input type="checkbox"  v-model="searchStore.unseen" label="Unseen" tip="Only show unseen emails."/>
        <Input type="checkbox"  v-model="searchStore.seen" label="Seen" tip="Only show seen emails."/>
        <Button expand @click="onSearch">Search</Button>
      </form>
    </div>
  </div>
</template>
<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
}

.modal-content {
  background: #f3f3f3;
  width: 100%;
  overflow-x: hidden;
  max-width: calc((400px * 2) + var(--s-spacing) + var(--m-spacing) * 2); /* (Size of field * 2 ) + spacing between fields + padding */
  max-height: 90vh;
  overflow-y: auto;
  padding: var(--m-spacing);
  box-sizing: border-box;
  border-radius: var(--l-border-radius);
  position: relative;

  display: flex;
  flex-direction: column;
}

.modal-content header {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;

  position: sticky;
  top: 0;
  background-color: #f3f3f3;

  z-index: 200;
  margin-bottom: var(--m-spacing);
}

.modal-content form {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  justify-content: start;
  gap: var(--s-spacing);
}

.close-btn {
  padding: 0;
  margin: 0;
  background: none;
  border: none;
  cursor: pointer;
  align-self: flex-end;
}
</style>