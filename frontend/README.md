# Frontend Sistem Kompresi PDF (FEPF)

Frontend untuk sistem pemrosesan paralel kompresi PDF menggunakan Vue 3 + Vite.

## Features

- 📤 Upload multiple PDF files (drag & drop / click to browse)
- 🔄 Pemrosesan paralel untuk kompresi multiple files
- 📊 Real-time progress tracking untuk setiap file
- 📈 Statistik kompresi (ukuran awal, akhir, pengurangan %, waktu proses)
- ⬇️ Download file hasil kompresi
- 🎨 UI yang modern dan responsif

## Setup

1. Install dependencies:
```bash
npm install
```

2. Copy file `.env.example` menjadi `.env` dan sesuaikan URL backend:
```bash
cp .env.example .env
```

3. Edit file `.env`:
```
VITE_API_URL=http://localhost:5000/api
```
Sesuaikan dengan URL backend yang dibuat teman Anda.

## Development

Jalankan development server:
```bash
npm run dev
```

Aplikasi akan berjalan di `http://localhost:3000`

## Build untuk Production

```bash
npm run build
```

File production akan ada di folder `dist/`

## Preview Build

```bash
npm run preview
```

## Integrasi dengan Backend

File `src/services/api.js` berisi fungsi-fungsi untuk komunikasi dengan backend:

- `compressMultiplePDFs()` - Mengirim multiple files untuk dikompres secara paralel
- `downloadCompressedFile()` - Download file hasil kompresi
- `getCompressionStatus()` - Cek status kompresi

### API Endpoints yang Dibutuhkan dari Backend:

1. **POST** `/api/compress` - Upload dan kompres PDF
   - Request: FormData dengan file PDF
   - Response: { compressedSize, reduction, processingTime, downloadUrl }

2. **GET** `/api/download/:fileId` - Download file hasil kompresi
   - Response: File binary

3. **GET** `/api/status/:taskId` - Cek status kompresi
   - Response: { status, progress, ... }

## Catatan untuk Integrasi Backend

Saat ini aplikasi berjalan dalam **mode simulasi**. Untuk menggunakan backend yang sesungguhnya:

1. Pastikan backend sudah berjalan
2. Update `VITE_API_URL` di file `.env`
3. Edit file `src/services/api.js`
4. Uncomment kode API call yang sesungguhnya
5. Hapus/comment kode simulasi

## Teknologi yang Digunakan

- Vue 3 - Framework JavaScript
- Vite - Build tool & dev server
- Axios - HTTP client
- CSS3 - Styling

## Struktur Project

```
fepf/
├── src/
│   ├── services/
│   │   └── api.js          # API service untuk backend
│   ├── App.vue             # Main component
│   ├── main.js             # Entry point
│   └── style.css           # Global styles
├── index.html
├── vite.config.js
├── package.json
└── README.md
```
