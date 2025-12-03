# PDF Compressor - Multiprocessing Project

Project kompresi PDF menggunakan backend Rust (multiprocessing) dan frontend Vue.js.

## 📁 Struktur Project

```
Project-Multiprocessing-PF-B-_5-dev/
├── backend/              # Rust backend (API + Worker)
│   ├── src/
│   │   ├── main_api.rs  # REST API server
│   │   ├── compressor.rs # Core compression logic
│   │   ├── worker.rs    # Worker processes
│   │   └── launcher.rs  # Job launcher
│   ├── Cargo.toml
│   └── Dockerfile
├── frontend/            # Vue.js frontend
│   ├── src/
│   │   ├── components/
│   │   │   ├── FileUpload.vue
│   │   │   ├── ProgressList.vue
│   │   │   └── FileTable.vue
│   │   ├── services/
│   │   │   └── api.js   # Backend API client
│   │   ├── App.vue
│   │   └── main.js
│   ├── package.json
│   ├── vite.config.js
│   └── Dockerfile
├── data/
│   ├── uploads/         # Upload folder
│   └── compressed/      # Hasil kompresi
└── docker-compose.yml   # Orchestration
```

## 🚀 Cara Menjalankan

### Option 1: Menggunakan Docker (Recommended)

1. **Build dan jalankan semua services:**
   ```bash
   cd Project-Multiprocessing-PF-B-_5-dev
   docker-compose up --build
   ```

2. **Akses aplikasi:**
   - Frontend: http://localhost:5173
   - Backend API: http://localhost:3000

3. **Stop services:**
   ```bash
   docker-compose down
   ```

### Option 2: Menjalankan Manual

#### Backend (Rust)

1. **Install Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Jalankan backend:**
   ```bash
   cd backend
   cargo build --release
   cargo run --bin main_api
   ```

   Backend akan berjalan di `http://localhost:3000`

#### Frontend (Vue.js)

1. **Install dependencies:**
   ```bash
   cd frontend
   npm install
   ```

2. **Jalankan development server:**
   ```bash
   npm run dev
   ```

   Frontend akan berjalan di `http://localhost:5173`

## 🔧 Konfigurasi

### Backend

- **Port:** 3000 (default)
- **Upload directory:** `./data/uploads`
- **Compressed directory:** `./data/compressed`
- **CORS:** Enabled untuk semua origin

### Frontend

- **Port:** 5173 (default)
- **Backend URL:** Konfigurasi di file `.env`
  ```env
  VITE_BACKEND_URL=http://localhost:3000
  ```

## 📡 API Endpoints

### POST /compress
Upload dan kompres PDF

**Request:**
- Method: POST
- Content-Type: multipart/form-data
- Body: file (PDF)

**Response:**
```json
{
  "success": true,
  "originalSize": 1048576,
  "compressedSize": 524288,
  "reduction": 50.0,
  "processingTime": 2.5,
  "downloadUrl": "http://localhost:3000/download/uuid-compressed-filename.pdf"
}
```

### GET /download/:file
Download file hasil kompresi

**Request:**
- Method: GET
- Path param: file (nama file)

**Response:**
- Content-Type: application/pdf
- Content-Disposition: attachment

## 🛠️ Tech Stack

### Backend
- **Rust** - Programming language
- **Axum** - Web framework
- **Tokio** - Async runtime
- **lopdf** - PDF manipulation
- **Rayon** - Parallel processing
- **Tower-http** - Middleware (CORS)

### Frontend
- **Vue 3** - Frontend framework
- **Vite** - Build tool
- **Axios** - HTTP client
- **Tailwind CSS** - Styling

## 📦 Build untuk Production

### Backend
```bash
cd backend
cargo build --release
# Binary ada di: target/release/main_api
```

### Frontend
```bash
cd frontend
npm run build
# Output ada di: dist/
```

## 🐳 Docker Commands

```bash
# Build semua images
docker-compose build

# Jalankan di background
docker-compose up -d

# Lihat logs
docker-compose logs -f

# Lihat logs backend saja
docker-compose logs -f backend

# Lihat logs frontend saja
docker-compose logs -f frontend

# Restart service tertentu
docker-compose restart backend

# Stop dan hapus containers
docker-compose down

# Stop, hapus containers + volumes
docker-compose down -v
```

## 📝 Development Notes

- Backend menggunakan multiprocessing dengan Rayon untuk parallel compression
- Frontend mengirim request ke backend melalui REST API
- File upload menggunakan FormData
- Progress tracking dilakukan di frontend
- CORS sudah dikonfigurasi untuk development

## 🔍 Troubleshooting

### Port sudah digunakan
```bash
# Cek port 3000
lsof -i :3000
# Cek port 5173
lsof -i :5173
# Kill process jika perlu
kill -9 <PID>
```

### Permission denied pada folder data
```bash
chmod -R 755 data/
```

### CORS error
Pastikan backend sudah running dan CORS layer sudah dikonfigurasi dengan benar di `main_api.rs`

## 📄 License

MIT

---

**Happy Coding! 🎉**
