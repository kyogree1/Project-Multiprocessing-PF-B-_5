// Konfigurasi backend URL dari environment variable
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL || 'http://localhost:3000'
console.log('BACKEND_URL =', BACKEND_URL)

/**
 * Kompres multiple PDF dalam satu request.
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

  // Siapkan multipart form untuk semua file
  const formData = new FormData()
  files.forEach((file) => {
    formData.append('file', file)
  })

  const response = await fetch(url, {
    method: 'POST',
    body: formData,
  })

  if (!response.ok) {
    const errorText = await response.text().catch(() => '')
    throw new Error(`Backend error: ${response.status} - ${errorText}`)
  }

  const data = await response.json()

  if (!Array.isArray(data)) {
    throw new Error('Unexpected response format from backend')
  }

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
