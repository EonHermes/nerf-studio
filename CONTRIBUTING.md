# Contributing to NeRF Studio 🎨

Thank you for your interest in contributing! This document provides guidelines and instructions.

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on what's best for the community

## Getting Started

1. **Fork** the repository
2. **Clone** your fork: `git clone https://github.com/EonHermes/nerf-studio.git`
3. **Create a branch**: `git checkout -b feature/your-feature-name`

## Development Setup

### Backend (Rust)

```bash
# Install dependencies
cargo build

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt
```

### Frontend (React)

```bash
cd frontend

# Install dependencies
npm install

# Run development server
npm run dev

# Run tests
npm test

# Lint code
npm run lint
```

## Pull Request Process

1. **Update documentation** if you're adding/changing features
2. **Add tests** for new functionality
3. **Ensure all checks pass**:
   - `cargo test` passes
   - `cargo clippy` has no warnings
   - Frontend tests pass
4. **Write a clear description** of your changes
5. **Link related issues** if applicable

## Coding Standards

### Rust

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use meaningful variable names
- Document public APIs with `///` comments
- Handle errors gracefully with `Result` types

### TypeScript/React

- Use TypeScript for type safety
- Follow React best practices
- Component files should be named PascalCase
- Use functional components with hooks

## Testing Guidelines

- Write unit tests for core logic
- Integration tests for API endpoints
- Frontend component tests where applicable
- Aim for >80% test coverage

## NeRF-Specific Contributions

If you're working on the ML/NeRF components:

1. **Performance matters**: GPU acceleration is critical
2. **Accuracy first**: Novel view quality is paramount
3. **Document algorithms**: Explain your approach clearly
4. **Benchmark**: Include performance comparisons

## Questions?

Open an issue for questions or join discussions in existing issues.

Happy coding! 🚀
