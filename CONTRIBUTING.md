# Contributing to Extro

Thank you for your interest in contributing to Extro! As a solo developer project with limited resources, community contributions are especially valuable. This document provides guidelines and instructions to help you get started.

## 🌟 Ways to Contribute

There are many ways to contribute to Extro, regardless of your experience level:

### 🧩 Code Contributions

1. **Platform Adapters**
   - Implement adapters for new platforms (e.g., Figma, Sketch, IntelliJ)
   - Enhance existing adapters with more features
   - Add support for platform-specific extension types

2. **Language Support**
   - Add support for new programming languages
   - Improve code generation for existing languages
   - Create language-specific optimizations

3. **Core Functionality**
   - Enhance the CLI interface
   - Improve the ExtroIntent intermediate representation
   - Optimize the direct code generation pipeline

4. **AI Integration**
   - Refine prompts for code generation
   - Integrate new AI models via Candle
   - Improve inference performance

### 📝 Documentation

- Improve existing documentation
- Create tutorials and examples
- Add docstrings to code
- Create diagrams explaining Extro's architecture

### 🧪 Testing

- Write unit tests
- Create integration tests
- Test extensions on different platforms
- Report bugs and edge cases

## 🚀 Getting Started

### Prerequisites

- Rust toolchain (latest stable)
- Cargo and rustc
- Git

### Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/extro.git
   cd extro
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/jitpomi/extro.git
   ```
4. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

### Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run Extro CLI
cargo run -- extend my-extension
```

## 🔍 Pull Request Process

1. Ensure your code follows our style guidelines
2. Update documentation as needed
3. Make sure all tests pass
4. Submit a pull request with a clear description of your changes

## 🐛 Good First Issues

If you're new to the project, here are some good issues to start with:

- **Documentation improvements**: Help clarify our docs
- **Platform adapter enhancements**: Add support for specific features in existing adapters
- **Test coverage**: Write tests for existing functionality
- **CLI improvements**: Enhance user experience and error messages

Check our [Issues](https://github.com/jitpomi/extro/issues) page for issues labeled `good-first-issue`.

## 🧠 Architecture Overview

Extro consists of several key components:

1. **EXTRO CLI**: Command-line interface for user interaction
2. **EXTRO BRAIN (Orchestrator)**: Central component coordinating all operations
3. **EXTRO INTENT (IR)**: Intermediate representation of extension intent
4. **AI ENGINE**: Direct code generation from intent
5. **PLATFORM ADAPTERS**: Platform-specific translation layers

For detailed architecture information, see [ARCHITECTURE.md](ARCHITECTURE.md).

## 💬 Communication

- GitHub Issues: For bug reports and feature requests
- Pull Requests: For code contributions
- Email: dev@jitpomi.com (response times may vary as this is a solo project)
- Twitter: [@ExtroDev](https://twitter.com/ExtroDev)

## 📜 Code of Conduct

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) before contributing.

## 🙏 Thank You

Your contributions help make Extro better for everyone. We appreciate your time and effort!
