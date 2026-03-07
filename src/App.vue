<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

let unlistenDrop: any; //Drag N Drop

let draggedFilePath: string;
let conversionStatus  = ref<string>("Drag a video in to convert it!");
const maxFileSize = ref<number | null>(10);
const selectedQualityOption = ref<string>('Source');
const qualityOptions = ['Source', '1440p', '1080p', '720p', '480p', '360p'];
const selectedFramerateOption = ref<string>('Source');
const framerateOptions = ['Source', '60', '30'];
const isHardwareAccelerated = ref(false);
const isModernCodec = ref(true);

onMounted(async () => {
  unlistenDrop = await listen('tauri://drag-drop', (event: any) => {
    draggedFilePath = event.payload.paths[0];
    conversionStatus.value = draggedFilePath.split(/[\\/]/).pop() ?? "";
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
      maxFileSize: maxFileSize.value,
      qualityOption: selectedQualityOption.value,
      framerateOption: selectedFramerateOption.value,
      isHardwareAccelerated: isHardwareAccelerated.value,
      isModernCodec: isModernCodec.value
    });

    conversionStatus.value = "Done! You can drag in another video to convert it.";
    draggedFilePath = "";
  } catch (error) {
    conversionStatus.value = "Conversion failed: " + error;
  }
}
</script>

<template>
  <main class="container">

    <p>{{conversionStatus}}</p>

    <span v-if="conversionStatus == `Converting...`" class="spinner"></span>

    <div>Size limit in MB:<input 
      id="numeric-input"
      v-model.number="maxFileSize" 
      type="number" 
      min="5"
      step="1"
      placeholder="10"
    />

    Quality:
    <select v-model="selectedQualityOption">
      <option v-for="item in qualityOptions" :key="item" :value="item">
        {{ item }}
      </option>
    </select>

    Framerate:
    <select v-model="selectedFramerateOption">
      <option v-for="item in framerateOptions" :key="item" :value="item">
        {{ item }}
      </option>
    </select>

    Use NVENC (Nvidia):
    <input type="checkbox" v-model="isHardwareAccelerated">

    Modern codec (h265):
    <input type="checkbox" v-model="isModernCodec">

    <button :disabled="!draggedFilePath" @click="transcodeVideo">Convert</button>
  </div>

  </main>
</template>

<style scoped>
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
  align-items: center;
}

.row {
  display: flex;
  justify-content: center;
}

.spinner {
  border: 0.5em solid rgba(0, 0, 0, 0.1);
  width: 2em;
  height: 2em;
  border-radius: 50%;
  border-left-color: #09d;
  animation: spin 1s ease infinite;
  display: inline-block;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
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
select,
checkbox,
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
  margin: 0.5em 0.5em 0 0;
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

input[type="number"] {
  width: 6ch; 
}

input,
select,
checkbox,
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