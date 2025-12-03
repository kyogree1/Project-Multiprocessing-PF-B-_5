<template>
  <div 
    class="border-2 border-dashed rounded-2xl py-16 px-8 text-center bg-white cursor-pointer transition-all duration-300 mb-8 shadow-md"
    :class="isDragging ? 'border-pink-dark bg-pink-100 scale-105 shadow-pink-primary/30' : 'border-pink-primary shadow-pink-primary/10 hover:border-pink-dark hover:bg-pink-light hover:shadow-lg hover:shadow-pink-primary/20'"
    @click="triggerFileInput"
    @drop.prevent="handleDrop"
    @dragover.prevent="isDragging = true"
    @dragleave.prevent="isDragging = false"
  >
    <div class="text-7xl mb-5 drop-shadow-md">📄</div>
    <div class="text-2xl text-gray-800 mb-3 font-semibold">Klik atau seret file PDF di sini</div>
    <div class="text-gray-500 text-base">Mendukung multiple file PDF</div>
    <input 
      type="file" 
      ref="fileInput"
      accept=".pdf"
      multiple
      class="hidden"
      @change="handleFileSelect"
    >
  </div>
</template>

<script>
import { ref } from 'vue'

export default {
  name: 'FileUpload',
  emits: ['filesAdded'],
  setup(props, { emit }) {
    const fileInput = ref(null)
    const isDragging = ref(false)

    const triggerFileInput = () => {
      fileInput.value.click()
    }

    const handleFileSelect = (event) => {
      const files = Array.from(event.target.files)
      if (files.length > 0) {
        emit('filesAdded', files)
        event.target.value = '' // Reset input
      }
    }

    const handleDrop = (event) => {
      isDragging.value = false
      const files = Array.from(event.dataTransfer.files).filter(
        file => file.type === 'application/pdf'
      )
      if (files.length > 0) {
        emit('filesAdded', files)
      }
    }

    return {
      fileInput,
      isDragging,
      triggerFileInput,
      handleFileSelect,
      handleDrop
    }
  }
}
</script>
