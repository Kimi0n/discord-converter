<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import './assets/style.css'

let unlistenDrop: any; //Drag N Drop

let draggedFilePath: string;
let conversionStatus  = ref<string>("Drag a video in to convert it!");
let isConverting = false;
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

listen('ffmpeg-progress', (event) => {
  const currentPercentString = event.payload; 
  conversionStatus.value = `Converting: ${currentPercentString}`;
  draggedFilePath = "";
});

listen('ffmpeg-error', (event) => {
  isConverting = false;

  const errorMessage = event.payload; 
  conversionStatus.value = `${errorMessage}`;
});

listen('ffmpeg-finished', (event) => {
  isConverting = false;
  const conversionEndCode = event.payload; 

  if(conversionEndCode == 0) {
    conversionStatus.value = `Done! Drag another video in to convert another one.`;
  }
});

async function transcodeVideo() {
  try {
    isConverting = true;

    await invoke('convert_video', { 
      filePath: draggedFilePath,
      maxFileSize: maxFileSize.value,
      qualityOption: selectedQualityOption.value,
      framerateOption: selectedFramerateOption.value,
      isHardwareAccelerated: isHardwareAccelerated.value,
      isModernCodec: isModernCodec.value
    });

  } catch (error) {
    isConverting = false;
    conversionStatus.value = "Conversion failed: " + error;
  }
}
</script>

<template>
  <main class="container">

    <p>{{conversionStatus}}</p>

    <span v-if="isConverting" class="spinner"></span>

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