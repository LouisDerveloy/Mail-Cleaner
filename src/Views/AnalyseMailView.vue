<script setup lang="ts">
import { ref } from "vue";
import { invoke, Channel } from "@tauri-apps/api/core";

const SenderList = ref<HTMLUListElement | null>(null);

const ret_channel = new Channel()
ret_channel.onmessage = (data: any) => {
  let sender;
  for (sender of data) {
    let element = document.createElement("li");
    element.textContent = sender;
    SenderList.value?.appendChild(element);
  }
}

function start_analyse() {
  invoke("get_list", {"retChannel": ret_channel})
}

</script>
<template>
<h1>This is the Mail analyse View</h1>
  <button @click="start_analyse">Analyse</button>
  <div class="SenderList">
    <ul ref="SenderList">
    </ul>
  </div>
</template>

<style scoped>
  .SenderList {
    height: 50vh;
    overflow: scroll;
  }

  .SenderList li {
    background-color: white;
    width: 100%;
    padding: .2rem;
    list-style-type: none;
  }

  .SenderList li:hover {
    background-color: #d9d9d9;
  }
</style>