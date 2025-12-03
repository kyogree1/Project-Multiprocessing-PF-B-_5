<template>
  <div v-if="files.length > 0" class="mt-8">
    <div class="flex justify-between items-center mb-5">
      <h3 class="text-gray-800 text-xl font-semibold">Daftar PDF ({{ files.length }})</h3>
      <button 
        @click="$emit('compressAll')" 
        :disabled="isProcessing"
        class="bg-gradient-to-br from-pink-primary to-pink-dark text-white border-none py-3 px-7 rounded-full cursor-pointer text-base font-semibold transition-all duration-300 shadow-lg shadow-pink-primary/30 hover:-translate-y-0.5 hover:shadow-xl hover:shadow-pink-primary/40 disabled:bg-gray-300 disabled:cursor-not-allowed disabled:transform-none disabled:shadow-none"
      >
        {{ isProcessing ? 'Memproses...' : 'Kompres Semua' }}
      </button>
    </div>

    <div class="bg-white rounded-xl overflow-hidden shadow-md shadow-pink-primary/10">
      <table class="w-full border-collapse">
        <thead class="bg-gradient-to-br from-pink-primary to-pink-dark text-white">
          <tr>
            <th class="p-4 text-left font-semibold text-sm">Nama File</th>
            <th class="p-4 text-left font-semibold text-sm">Ukuran</th>
            <th class="p-4 text-left font-semibold text-sm">Status</th>
            <th class="p-4 text-left font-semibold text-sm">Aksi</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(file, index) in files" :key="index" class="border-b border-pink-100 transition-colors hover:bg-pink-light">
            <td class="p-4">
              <div class="flex items-center gap-2.5 font-medium">
                <span class="text-xl">📄</span>
                <span class="text-gray-800">{{ file.name }}</span>
              </div>
            </td>
            <td class="p-4 text-gray-800">{{ formatFileSize(file.size) }}</td>
            <td class="p-4">
              <span 
                class="inline-block py-1.5 px-3.5 rounded-full text-xs font-semibold"
                :class="getStatusClass(file.status)"
              >
                {{ getStatusText(file.status) }}
              </span>
            </td>
            <td class="p-4">
              <div class="flex gap-2">
                <button 
                  v-if="file.status === 'completed' && file.downloadUrl"
                  @click="$emit('download', file)"
                  title="Download"
                  class="bg-white border-2 border-green-500 py-2 px-3 rounded-lg cursor-pointer text-lg transition-all duration-200 hover:bg-green-500 hover:scale-110"
                >
                  ⬇️
                </button>
                <button 
                  v-if="!file.status || file.status === 'error'"
                  @click="$emit('remove', index)"
                  title="Hapus"
                  class="bg-white border-2 border-pink-primary py-2 px-3 rounded-lg cursor-pointer text-lg transition-all duration-200 hover:bg-pink-primary hover:scale-110"
                >
                  🗑️
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template><script>
export default {
  name: 'FileTable',
  props: {
    files: {
      type: Array,
      required: true
    },
    isProcessing: {
      type: Boolean,
      default: false
    }
  },
  emits: ['compressAll', 'download', 'remove'],
  methods: {
    formatFileSize(bytes) {
      if (!bytes || bytes === 0 || isNaN(bytes)) return '0 Bytes'
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
      return statusMap[status] || 'Belum Diproses'
    },
    getStatusClass(status) {
      const classes = {
        'pending': 'bg-gray-100 text-gray-600',
        'queued': 'bg-yellow-100 text-yellow-700',
        'processing': 'bg-blue-100 text-blue-700 animate-pulse',
        'completed': 'bg-green-100 text-green-700',
        'error': 'bg-red-100 text-red-700'
      }
      return classes[status] || 'bg-gray-100 text-gray-600'
    }
  }
}
</script>
