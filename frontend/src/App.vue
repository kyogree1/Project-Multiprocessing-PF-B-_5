<template>
  <div class="min-h-screen bg-white">
    <div class="bg-gradient-to-br from-pink-primary to-pink-dark text-white py-8 px-5 text-center mb-10">
      <h1 class="text-4xl font-semibold mb-2.5">📄 Kompres PDF</h1>
      <p class="text-base opacity-95">Kompres file PDF dengan mudah dan cepat</p>
    </div>
    
    <div class="max-w-3xl mx-auto px-5 pb-10">
      <!-- Upload Component -->
      <FileUpload @filesAdded="handleFilesAdded" />

      <!-- File Table Component -->
      <FileTable 
        :files="fileList"
        :isProcessing="isProcessing"
        @compressAll="compressFiles"
        @download="downloadFile"
        @remove="removeFile"
      />

      <!-- Progress Component -->
      <ProgressList :progressItems="compressionProgress" />
    </div>
  </div>
</template>

<script>
import { ref } from 'vue'
import FileUpload from './components/FileUpload.vue'
import FileTable from './components/FileTable.vue'
import ProgressList from './components/ProgressList.vue'
import { compressMultiplePDFs } from './services/api'

export default {
  name: 'App',
  components: {
    FileUpload,
    FileTable,
    ProgressList
  },
  setup() {
    const fileList = ref([])
    const isProcessing = ref(false)
    const compressionProgress = ref([])

    const handleFilesAdded = (files) => {
      // Add files with initial status and ensure size property is preserved
      const newFiles = files.map(file => ({
        name: file.name,
        size: file.size,
        type: file.type,
        lastModified: file.lastModified,
        file: file, // Keep reference to original File object
        status: null,
        downloadUrl: null
      }))
      fileList.value = [...fileList.value, ...newFiles]
    }

    const removeFile = (index) => {
      fileList.value.splice(index, 1)
    }

    const compressFiles = async () => {
      if (fileList.value.length === 0) return

      isProcessing.value = true
      
      // Initialize progress for all files
      compressionProgress.value = fileList.value.map(file => ({
        fileName: file.name,
        status: 'queued',
        progress: 0,
        stats: null,
        downloadUrl: null,
        error: null
      }))

      // Update file list with queued status
      fileList.value = fileList.value.map(file => ({
        ...file,
        status: 'queued'
      }))

      try {
        // Call API service with original File objects
        const originalFiles = fileList.value.map(item => item.file)
        await compressMultiplePDFs(
          originalFiles,
          (index, progress, status, stats, downloadUrl, error) => {
            // Update progress
            if (index >= 0 && index < compressionProgress.value.length) {
              compressionProgress.value[index] = {
                ...compressionProgress.value[index],
                progress,
                status,
                stats,
                downloadUrl,
                error
              }
              
              // Update file list status
              if (index < fileList.value.length) {
                fileList.value[index] = {
                  ...fileList.value[index],
                  status,
                  downloadUrl
                }
              }
            }
          }
        )
      } catch (error) {
        console.error('Error during compression:', error)
        alert('Terjadi kesalahan saat kompresi: ' + error.message)
      } finally {
        isProcessing.value = false
      }
    }

    const downloadFile = (file) => {
      if (file.downloadUrl) {
        const link = document.createElement('a')
        link.href = file.downloadUrl
        link.download = file.name.replace('.pdf', '_compressed.pdf')
        document.body.appendChild(link)
        link.click()
        document.body.removeChild(link)
      }
    }

    return {
      fileList,
      isProcessing,
      compressionProgress,
      handleFilesAdded,
      removeFile,
      compressFiles,
      downloadFile
    }
  }
}
</script>
