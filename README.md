# NeRF Studio 🎨

**Neural Radiance Fields Studio** - Create photorealistic 3D scenes from 2D photo collections using cutting-edge neural radiance field technology.

![Status](https://img.shields.io/badge/status-beta-blue)
![Rust](https://img.shields.io/badge/rust-1.75+-orange)
![React](https://img.shields.io/badge/react-18.2.0-61dafb)
![License](https://img.shields.io/badge/license-MIT-green)

## Features ✨

- **📷 Photo Upload**: Upload collections of photos taken from different angles
- **🤖 ML Training**: Train NeRF models using neural networks (candle/tch-rs integration ready)
- **🎨 Novel View Synthesis**: Render photorealistic views from arbitrary camera positions
- **📊 3D Visualization**: Interactive 3D viewer with Three.js/React Three Fiber
- **💾 Export**: Export to standard 3D formats (OBJ, GLTF, GLB, PLY)
- **⚡ GPU Acceleration**: Ready for CUDA/GPU acceleration in production

## Tech Stack 🛠️

### Backend
- **Rust** - High-performance server with Axum framework
- **SQLx** - Async SQLite database with type-safe queries
- **Tokio** - Asynchronous runtime
- **Nalgebra** - 3D math and linear algebra
- **Image** - Image processing and thumbnail generation

### Frontend
- **React 18** + TypeScript
- **Three.js** + React Three Fiber for 3D visualization
- **Axios** - HTTP client
- **Vite** - Fast build tool

## Quick Start 🚀

### Prerequisites

- Rust 1.75+ 
- Node.js 18+
- SQLite 3.x

### Backend Setup

```bash
# Install dependencies
cargo install

# Initialize database
cargo run -- init

# Run server
cargo run -- serve --port 3000
```

### Frontend Setup

```bash
cd frontend

# Install dependencies
npm install

# Development server
npm run dev

# Build for production
npm run build
```

## API Reference 📚

### Scenes

#### Create Scene
```http
POST /api/v1/scenes
Content-Type: application/json

{
  "name": "My Beautiful Scene",
  "description": "A photorealistic 3D reconstruction"
}
```

#### List Scenes
```http
GET /api/v1/scenes
```

#### Get Scene
```http
GET /api/v1/scenes/:id
```

### Images

#### Upload Images
```http
POST /api/v1/images/upload
Content-Type: multipart/form-data

files: [image1.jpg, image2.jpg, ...]
```

#### Download Image
```http
GET /api/v1/images/:id/download
```

### Rendering

#### Render Novel View
```http
POST /api/v1/render
Content-Type: application/json

{
  "scene_id": "uuid",
  "camera_position": [0.0, 0.0, 3.0],
  "camera_rotation": [0.0, 0.0],
  "width": 512,
  "height": 512
}
```

### Export

#### Export to 3D Format
```http
POST /api/v1/export
Content-Type: application/json

{
  "scene_id": "uuid",
  "format": "gltf",
  "include_textures": true
}
```

## Architecture 🏗️

```
nerf-studio/
├── src/
│   ├── api/           # HTTP API handlers
│   │   ├── scenes.rs
│   │   ├── images.rs
│   │   ├── render.rs
│   │   └── export.rs
│   ├── models/        # Data structures
│   ├── nerf/          # NeRF engine & algorithms
│   │   ├── engine.rs  # Training & inference
│   │   ├── camera.rs  # Camera utilities
│   │   └── volume.rs  # Volume rendering
│   ├── utils/         # Helper functions
│   └── main.rs        # Entry point
├── frontend/          # React application
│   ├── src/
│   │   ├── api/       # API client
│   │   ├── components/# React components
│   │   ├── pages/     # Page components
│   │   └── App.tsx
│   └── package.json
├── migrations/        # Database migrations
├── Cargo.toml
└── README.md
```

## NeRF Algorithm Overview 🧠

Neural Radiance Fields represent 3D scenes as continuous volumetric functions. Given a 5D coordinate (x, y, z, θ, φ), the network outputs volume density σ and view-dependent color c.

### Key Components

1. **Positional Encoding**: Transforms input coordinates to high-frequency features
2. **Volume Rendering**: Integrates color along camera rays
3. **Hierarchical Sampling**: Coarse-to-fine sampling for efficiency
4. **Camera Models**: Pinhole camera with intrinsics and extrinsics

### Training Pipeline

```
Photos → Camera Pose Estimation → NeRF Training → Novel View Synthesis
```

## Development 📝

### Running Tests

```bash
# Backend tests
cargo test

# Frontend tests
cd frontend && npm test
```

### Code Quality

```bash
# Rust linting
cargo clippy
cargo fmt --check

# TypeScript linting
cd frontend && npm run lint
```

## Production Deployment 🌐

### Docker

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/nerf-server /usr/local/bin/
EXPOSE 3000
CMD ["nerf-server", "serve"]
```

### Environment Variables

```bash
DATABASE_URL="sqlite:nerf_studio.db?mode=rwc"
UPLOADS_DIR="./uploads"
PORT=3000
RUST_LOG="info"
```

## Contributing 🤝

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License 📄

MIT License - see [LICENSE](LICENSE) file for details

## Acknowledgments 🙏

- Original NeRF paper: [Mildenhall et al., 2020](https://www.matthewtancik.com/nerf)
- Rust community for amazing crates
- React team for the wonderful framework

---

Built with ❤️ by EonHermes
