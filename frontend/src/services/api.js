// Konfigurasi backend URL dari environment variable
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL || 'http://localhost:3000'
console.log('BACKEND_URL =', BACKEND_URL)

/**
 * Kompres multiple PDF dalam satu request dengan simulasi progress visual.
 *
 * @param {File[]} files - Array file PDF
 * @param {'single'|'rayon'} mode - mode kompresi
 * @param {Function} onProgress - (index, progress, status, stats, downloadUrl, error)
 */
export async function compressMultiplePDFs(files, mode, onProgress) {
  if (!Array.isArray(files) || files.length === 0) {
    return []
  }

  // Normalisasi mode
  const normalizedMode = mode === 'single' ? 'single' : 'rayon'
  const pathMode = normalizedMode === 'single' ? 'single' : 'rayon'
  const url = `${BACKEND_URL}/compress/${pathMode}`

  console.log('compressMultiplePDFs mode =', normalizedMode, 'url =', url)

  // Set awal: semua file masuk status processing (0%)
  files.forEach((_, index) => {
    if (typeof onProgress === 'function') {
      onProgress(index, 0, 'processing', null, null, null)
    }
  })

  // Estimasi waktu kompresi per file (dalam ms) - bisa disesuaikan
  const estimatedTimePerFile = normalizedMode === 'rayon' ? 3000 : 5000
  const progressUpdateInterval = 100 // Update progress setiap 100ms

  // Start simulasi progress untuk semua file
  const progressIntervals = files.map((file, index) => {
    return startProgressSimulation(index, estimatedTimePerFile, progressUpdateInterval, onProgress)
  })

  // Siapkan multipart form untuk semua file
  const formData = new FormData()
  files.forEach((file) => {
    formData.append('file', file)
  })

  let data
  try {
    const response = await fetch(url, {
      method: 'POST',
      body: formData,
    })

    if (!response.ok) {
      const errorText = await response.text().catch(() => '')
      throw new Error(`Backend error: ${response.status} - ${errorText}`)
    }

    data = await response.json()

    if (!Array.isArray(data)) {
      throw new Error('Unexpected response format from backend')
    }
  } finally {
    // Stop semua simulasi progress
    progressIntervals.forEach(interval => clearInterval(interval))
  }

  // Set progress ke 100% dan update stats
  data.forEach((jobData, index) => {
    if (index >= files.length) {
      return
    }

    const stats = {
      originalSize: jobData.originalSize,
      compressedSize: jobData.compressedSize,
      reduction: jobData.reductionPercent.toFixed(2),
      processingTime: jobData.processingTime.toFixed(3),
    }

    if (typeof onProgress === 'function') {
      onProgress(index, 100, 'completed', stats, jobData.downloadUrl, null)
    }
  })

  return data
}

/**
 * Simulasi progress yang smooth untuk satu file
 * @param {number} fileIndex - Index file
 * @param {number} estimatedTime - Estimasi waktu dalam ms
 * @param {number} updateInterval - Interval update dalam ms
 * @param {Function} onProgress - Callback progress
 * @returns {number} Interval ID
 */
function startProgressSimulation(fileIndex, estimatedTime, updateInterval, onProgress) {
  let currentProgress = 0
  const incrementPerUpdate = (95 / estimatedTime) * updateInterval // Max 95% saat processing
  
  const interval = setInterval(() => {
    currentProgress += incrementPerUpdate
    
    // Cap at 95% - sisanya akan di-set ke 100% saat selesai
    if (currentProgress >= 95) {
      currentProgress = 95
      clearInterval(interval)
    }
    
    if (typeof onProgress === 'function') {
      onProgress(fileIndex, Math.floor(currentProgress), 'processing', null, null, null)
    }
  }, updateInterval)
  
  return interval
}
