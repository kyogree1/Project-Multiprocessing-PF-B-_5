<template>
  <div class="min-h-screen bg-white">
    <div class="bg-gradient-to-br from-pink-primary to-pink-dark text-white py-8 px-5 text-center mb-10">
      <h1 class="text-4xl font-semibold mb-2.5">📄 Kompres PDF</h1>
      <p class="text-base opacity-95">Kompres file PDF dengan mudah dan cepat</p>
    </div>
    
    <div class="max-w-3xl mx-auto px-5 pb-10">
      <!-- Upload Component -->
      <FileUpload @filesAdded="handleFilesAdded" />

      <!-- Tambahan kecil: pilihan mode single / multi-thread (Rayon) -->
      <div class="flex justify-end items-center mt-4 mb-2 text-sm text-slate-700">
        <label class="flex items-center gap-2">
          <span>Mode kompresi:</span>
          <select
            v-model="mode"
            class="border border-slate-300 rounded-md px-2 py-1 text-sm bg-white"
          >
            <option value="rayon">Multi-thread (Rayon)</option>
            <option value="single">Single-thread</option>
          </select>
        </label>
      </div>

      <!-- File Table Component -->
      <FileTable 
        :files="fileList"
        :isProcessing="isProcessing"
        @compressAll="compressFiles"
        @download="downloadFile"
        @remove="removeFile"
      />

      <!-- Ringkasan waktu kompres total -->
      <div
        v-if="totalDuration !== null"
        class="mt-3 mb-2 text-right text-xs text-slate-700"
      >
        <span>
          Total waktu kompres
          ({{ lastModeUsed === 'rayon' ? 'Multi-thread (Rayon)' : 'Single-thread' }},
          {{ lastSuccessCount }} file):
        </span>
        <span class="font-semibold">
          {{ totalDuration.toFixed(3) }} detik
        </span>
      </div>

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
    const mode = ref('rayon') // 'rayon' | 'single'

    // untuk perbandingan single vs multi
    const totalDuration = ref(null)      // dalam detik (number | null)
    const lastModeUsed = ref(null)       // 'rayon' | 'single' | null
    const lastSuccessCount = ref(0)      // jumlah file yang berhasil dikompres

    const handleFilesAdded = (files) => {
      const newFiles = files.map(file => ({
        name: file.name,
        size: file.size,
        type: file.type,
        lastModified: file.lastModified,
        file: file,
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
      
      compressionProgress.value = fileList.value.map(file => ({
        fileName: file.name,
        status: 'queued',
        progress: 0,
        stats: null,
        downloadUrl: null,
        error: null
      }))

      fileList.value = fileList.value.map(file => ({
        ...file,
        status: 'queued'
      }))

      try {
        const originalFiles = fileList.value.map(item => item.file)
        console.log('compressFiles mode =', mode.value)

        const startedAt = performance.now()

        await compressMultiplePDFs(
          originalFiles,
          mode.value, // 'rayon' atau 'single'
          (index, progress, status, stats, downloadUrl, error) => {
            if (index >= 0 && index < compressionProgress.value.length) {
              compressionProgress.value[index] = {
                ...compressionProgress.value[index],
                progress,
                status,
                stats,
                downloadUrl,
                error
              }
              
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

        const elapsedMs = performance.now() - startedAt
        const successCount = compressionProgress.value.filter(
          item => item.status === 'completed'
        ).length

        totalDuration.value = elapsedMs / 1000
        lastModeUsed.value = mode.value
        lastSuccessCount.value = successCount
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
      mode,
      totalDuration,
      lastModeUsed,
      lastSuccessCount,
      handleFilesAdded,
      removeFile,
      compressFiles,
      downloadFile
    }
  }
}
</script>
