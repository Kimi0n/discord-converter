<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

let unlistenDrop: any; //TODO: change all : any file actual datatypes
const maxFileSize = ref<number | null>(10);
let draggedFilePath: string;
let conversionStatus  = ref<string>("Drag a video in to convert it!");

onMounted(async () => {
  unlistenDrop = await listen('tauri://drag-drop', (event: any) => {
    draggedFilePath = event.payload.paths[0];
    conversionStatus.value = draggedFilePath;
  });
});

onUnmounted(() => {
  if (unlistenDrop) unlistenDrop();
});

async function transcodeVideo() {
  conversionStatus.value = "Converting...";

  try {
    await invoke('convert_video', { 
      filePath: draggedFilePath,
      maxFileSize: maxFileSize.value
    });
    conversionStatus.value = "Done! You can drag in another video to convert it.";
    draggedFilePath = "";
  } catch (error) {
    conversionStatus.value = "conversion failed: " + error;
  }
  // const command = Command.sidecar('bin/ffmpeg', ['-i', 'input.mp4', 'output.avi']);
  // const output = await command.execute();
}
</script>

<template>
  <main class="container">
    <p>{{conversionStatus}}</p>
    <div>Size limit in MB:<input 
      id="numeric-input"
      v-model.number="maxFileSize" 
      type="number" 
      min="5"
      step="1"
      placeholder="10"
    />
    <button :disabled="!draggedFilePath" @click="transcodeVideo">Convert</button>
  </div>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #727272;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  margin-left: 0.3vw;
}

button:not(:disabled) {
  cursor: pointer;
  color: #0f0f0f;
}

button:not(:disabled):hover {
  border-color: #396cd8;
}
button:not(:disabled):active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>