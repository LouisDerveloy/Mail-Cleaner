<!--
  - Copyright (c) 2025. Louis DERVELOY (https://github.com/LouisDerveloy/)
  - Licensed under the PolyForm Noncommercial License 1.0.0
  - SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
  - See the LICENSE file in the project root for details.
  -->

<script lang="ts" setup>
import {computed} from 'vue';
import TipTool from '../Components/TipTool.vue';
import CrossIco from '../assets/images/cross.svg';

const props = defineProps<{
  modelValue: string | boolean | number;
  label: string;
  tip?: string;
  placeholder?: string;
  type: 'text' | 'date' | 'number' | 'checkbox' | 'password';
  max?: number;
  min?: number;
}>();

const emit = defineEmits(['update:modelValue']);

const textValue = computed({
  get: () => props.modelValue as string,
  set: (value) => emit('update:modelValue', value),
});

const boolValue = computed({
  get: () => props.modelValue as boolean,
  set: (value) => emit('update:modelValue', value),
});

const numberModel = computed({
  get: () => props.modelValue as number,
  set: (value) => emit('update:modelValue', value),
})

const dateValue = computed({
  get() {
    if (typeof props.modelValue === 'string' && props.modelValue) {
      // Convert from dd-Mon-yyyy to yyyy-mm-dd for the date input
      const parts = props.modelValue.split('-');
      if (parts.length === 3) {
        const day = parts[0].padStart(2, '0');
        const monthStr = parts[1];
        const year = parts[2];
        const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
        const monthIndex = months.findIndex(m => m.toLowerCase() === monthStr.toLowerCase());
        if (monthIndex !== -1) {
          const month = String(monthIndex + 1).padStart(2, '0');
          return `${year}-${month}-${day}`;
        }
      }
    }
    return ''; // Return empty if format is wrong or value is empty
  },
  set(newValue) {
    if (newValue) {
      // Convert from yyyy-mm-dd to dd-Mon-yyyy for the store
      const date = new Date(newValue);
      const day = date.getUTCDate();
      const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
      const month = months[date.getUTCMonth()];
      const year = date.getUTCFullYear();
      emit('update:modelValue', `${day}-${month}-${year}`);
    } else {
      emit('update:modelValue', '');
    }
  }
});

function clearInput() {
  emit('update:modelValue', '');
}
</script>

<template>
  <div :class="['form-group', {'bool' : props.type === 'checkbox'}]">
    <label>
      {{ label }}
      <TipTool v-if="typeof props.tip !== 'undefined'" :text="tip!"/>
    </label>

    <div v-if="props.type === 'checkbox'" class="input bool">
      <input v-model="boolValue" class="checkbox-input" type="checkbox"/>
    </div>
    <div v-else-if="props.type === 'date'" class="input">
      <input v-model="dateValue" type="date"/>
      <img v-if="modelValue" :src="CrossIco" class="clear-btn" @click="clearInput"/>
    </div>
    <div v-else-if="props.type === 'text' || props.type === 'password'" class="input">
      <input v-model="textValue"
             :placeholder="typeof props.placeholder === 'undefined' ? props.label : props.placeholder"
             :type="props.type"
      />
      <img v-if="modelValue" :src="CrossIco" class="clear-btn" @click="clearInput"/>
    </div>
    <div v-else-if="props.type === 'number'" class="input">
      <input v-model="numberModel"  :min="props.min" :max="props.max" :placeholder="typeof props.placeholder === 'undefined' ? props.label : props.placeholder"
             type="number">
    </div>
  </div>
</template>

<style scoped>
.form-group {
  width: 100%;
  max-width: 400px;
  margin-bottom: var(--s-spacing);
  display: flex;
  flex-direction: column;
}

label {
  color: var(--background-color-text);
  font-weight: bold;
  margin-bottom: var(--xs-spacing);
  display: flex;
  align-items: center;
  gap: var(--xs-spacing);
}

.input {
  display: flex;
  flex-direction: row;
  width: 100%;
  gap: var(--xs-spacing);
  flex-wrap: nowrap;
  padding: var(--xs-spacing);
  border: 1px solid #151515;
  border-radius: var(--s-border-radius);
  align-items: center;
}

.input input {
  background: none;
  border: none;
  outline: none;
  width: 100%;
  color: #333;
}

.clear-btn {
  cursor: pointer;
  padding: 0 var(--xs-spacing);
  stroke: #888;
  height: 16px;
}

.checkbox-input {
  height: 20px;
  width: 20px;
}

input[type="date"]::-webkit-calendar-picker-indicator {
  cursor: pointer;
  filter: invert(0.5);
}

/* We totally change the behavior if the input is supposed to be a check box */
.form-group.bool {
  flex-direction: row-reverse;
  align-items: center;
  justify-content: start;
  gap: var(--xs-spacing);
  width: min-content;
}

.form-group.bool .input {
  border: none;
  padding: 0;
  margin: 0;
}

.form-group.bool .input input {
  height: 20px;
  aspect-ratio: 1/1;
  accent-color: var(--primary-color);
}

.form-group.bool label {
  margin: 0;
}

.form-group.bool label span {
  margin: 0;
}
</style>