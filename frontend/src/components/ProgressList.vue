<template>
  <div v-if="progressItems.length > 0" class="mt-10">
    <h3 class="text-gray-800 text-xl mb-5 font-semibold">Progress Kompresi</h3>
    
    <div 
      v-for="(item, index) in progressItems" 
      :key="index" 
      class="bg-white rounded-xl p-5 mb-4 border border-pink-100 shadow-sm shadow-pink-primary/10"
    >
      <div class="flex justify-between items-center mb-3">
        <strong class="text-gray-800">{{ item.fileName }}</strong>
        <span 
          class="text-sm font-semibold py-1 px-3 rounded-xl"
          :class="{
            'bg-blue-100 text-blue-700': item.status === 'processing',
            'bg-green-100 text-green-700': item.status === 'completed',
            'bg-red-100 text-red-700': item.status === 'error'
          }"
        >
          {{ getStatusText(item.status) }}
        </span>
      </div>
      
      <div class="bg-gray-200 rounded-lg h-5 overflow-hidden mb-2">
        <div class="bg-gradient-to-r from-pink-primary to-pink-dark h-full transition-all duration-300 rounded-lg" :style="{ width: item.progress + '%' }"></div>
      </div>
      
      <div class="text-sm text-gray-600 mb-2.5">{{ item.progress }}%</div>

      <!-- Compression Stats -->
      <div v-if="item.status === 'completed' && item.stats" class="grid grid-cols-2 md:grid-cols-4 gap-3 mt-4">
        <div class="bg-gray-50 p-3 rounded-lg text-center border border-gray-200">
          <div class="text-xs text-gray-600 mb-1.5">Ukuran Awal</div>
          <div class="text-base font-semibold text-pink-dark">{{ formatFileSize(item.stats.originalSize) }}</div>
        </div>
        <div class="bg-gray-50 p-3 rounded-lg text-center border border-gray-200">
          <div class="text-xs text-gray-600 mb-1.5">Ukuran Akhir</div>
          <div class="text-base font-semibold text-pink-dark">{{ formatFileSize(item.stats.compressedSize) }}</div>
        </div>
        <div class="bg-gray-50 p-3 rounded-lg text-center border border-gray-200">
          <div class="text-xs text-gray-600 mb-1.5">Pengurangan</div>
          <div class="text-base font-semibold text-pink-dark">{{ item.stats.reduction }}%</div>
        </div>
        <div class="bg-gray-50 p-3 rounded-lg text-center border border-gray-200">
          <div class="text-xs text-gray-600 mb-1.5">Waktu Proses</div>
          <div class="text-base font-semibold text-pink-dark">{{ item.stats.processingTime }}s</div>
        </div>
      </div>

      <!-- Error Message -->
      <div v-if="item.status === 'error' && item.error" class="bg-red-50 text-red-800 p-3 rounded-lg mt-2.5 border border-red-200 text-sm">
        {{ item.error }}
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'ProgressList',
  props: {
    progressItems: {
      type: Array,
      required: true
    }
  },
  methods: {
    formatFileSize(bytes) {
      if (bytes === 0) return '0 Bytes'
      const k = 1024
      const sizes = ['Bytes', 'KB', 'MB', 'GB']
      const i = Math.floor(Math.log(bytes) / Math.log(k))
      return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
    },
    getStatusText(status) {
      const statusMap = {
        'queued': 'Menunggu',
        'processing': 'Memproses',
        'completed': 'Selesai',
        'error': 'Error'
      }
      return statusMap[status] || status
    }
  }
}
</script>
