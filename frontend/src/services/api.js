// Konfigurasi backend URL dari environment variable
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL || 'http://localhost:3000'

/**
 * Poll job status sampai selesai
 */
async function pollJobStatus(jobId, index, onProgress) {
  const maxAttempts = 120 // 2 menit max (120 x 1 detik)
  let attempts = 0
  
  while (attempts < maxAttempts) {
    const response = await fetch(`${BACKEND_URL}/jobs/${jobId}`)
    
    if (!response.ok) {
      throw new Error(`Failed to get job status: ${response.status}`)
    }
    
    const jobData = await response.json()
    
    // Update progress based on status
    if (jobData.status === 'pending') {
      onProgress(index, 60, 'processing', null, null, null)
    } else if (jobData.status === 'processing') {
      onProgress(index, 75, 'processing', null, null, null)
    } else if (jobData.status === 'done') {
      onProgress(index, 95, 'processing', null, null, null)
      return jobData
    } else if (jobData.status === 'error') {
      throw new Error('Job processing failed on server')
    }
    
    // Wait 1 second before next poll
    await new Promise(resolve => setTimeout(resolve, 1000))
    attempts++
  }
  
  throw new Error('Job processing timeout')
}

/**
 * Kompres multiple PDF secara paralel menggunakan backend
 * @param {File[]} files - Array file PDF
 * @param {Function} onProgress - Callback: (index, progress, status, stats, downloadUrl, error)
 */
export async function compressMultiplePDFs(files, onProgress) {
  const compressionPromises = files.map((file, index) => 
    compressSinglePDF(file, index, onProgress)
  )
  
  return Promise.all(compressionPromises)
}

/**
 * Kompres single PDF menggunakan backend API
 */
async function compressSinglePDF(file, index, onProgress) {
  try {
    // Update status: mulai processing
    onProgress(index, 0, 'processing', null, null, null)

    // Buat FormData untuk upload
    const formData = new FormData()
    formData.append('file', file)

    // Kirim ke backend
    const response = await fetch(`${BACKEND_URL}/compress`, {
      method: 'POST',
      body: formData,
    })

    if (!response.ok) {
      const errorText = await response.text()
      throw new Error(`Backend error: ${response.status} - ${errorText}`)
    }

    const data = await response.json()

    if (!data.success) {
      throw new Error('Compression failed')
    }

    onProgress(index, 60, 'processing', null, null, null)
    
    // Poll untuk status job sampai selesai
    const jobId = data.jobId
    const jobData = await pollJobStatus(jobId, index, onProgress)
    
    // Format stats dari response backend
    const stats = {
      originalSize: jobData.originalSize,
      compressedSize: jobData.compressedSize,
      reduction: jobData.reductionPercent.toFixed(2),
      processingTime: jobData.processingTime.toFixed(3)
    }
    
    // Update status: selesai
    onProgress(index, 100, 'completed', stats, jobData.downloadUrl, null)
    
    return jobData

  } catch (error) {
    console.error(`Error compressing ${file.name}:`, error)
    onProgress(index, 0, 'error', null, null, error.message || 'Gagal mengkompresi file')
    throw error
  }
}

export default {
  compressMultiplePDFs
}
