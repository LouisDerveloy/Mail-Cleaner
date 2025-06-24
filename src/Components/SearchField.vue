<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  modelValue: string | boolean;
  label: string;
  isDate?: boolean;
  tip: string;
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
  <div class="form-group">
    <label>
      {{ label }}
      <TipTool :text="tip" />
    </label>
    <div class="input">
      <template v-if="typeof modelValue === 'boolean'">
        <input v-model="boolValue" type="checkbox" class="checkbox-input" />
      </template>
      <template v-else-if="isDate">
        <input v-model="dateValue" type="date" />
        <span v-if="modelValue" @click="clearInput" class="clear-btn">X</span>
      </template>
      <template v-else>
        <input v-model="textValue" type="text" :placeholder="label" />
        <span v-if="modelValue" @click="clearInput" class="clear-btn">X</span>
      </template>
    </div>
  </div>
</template>

<style scoped>
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

.input {
  display: flex;
  flex-direction: row;
  width: 100%;
  gap: .25rem;
  flex-wrap: nowrap;
  padding: .25rem;
  border-bottom: 1px solid #151515;
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
  padding: 0 .5rem;
  font-weight: bold;
  color: #888;
}

.checkbox-input {
  height: 20px;
  width: 20px;
}

input[type="date"]::-webkit-calendar-picker-indicator {
    cursor: pointer;
    filter: invert(0.5);
}
</style> 