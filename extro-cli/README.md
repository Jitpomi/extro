# Extro: Universal Extension Framework

Extro is an AI-powered universal extension framework enabling developers to create, transform, and optimize plugins for any platform using any language. With the motto "Write Once, Extend Everywhere," Extro uses AI-driven code generation to eliminate boilerplate and platform-specific complexity.

## Features

- **AI-Powered Code Generation**: Direct code generation without templates
- **Cross-Platform**: Support for VSCode, Chrome, Blender, JetBrains, Godot, Figma, and more
- **Cross-Language**: Generate extensions in JavaScript, Python, C++, GDScript, and other languages
- **Intent-Driven**: Abstract your extension's purpose, not just its implementation

## Getting Started

1. Install Extro:
   ```bash
   cargo install extro
   ```

2. Create a new extension:
   ```bash
   extro extend my-extension
   ```
   Follow the interactive prompts to select your target platform, extension type, and language.

3. Transform an existing extension to another platform:
   ```bash
   extro transform --source ./my-vscode-extension --target chrome
   ```

4. Optimize your extension:
   ```bash
   extro optimize ./my-extension
   ```

## Project Structure

- `extro-cli`: Main CLI application
  - `src/main.rs`: CLI entry point and command routing
  - `src/commands/`: Command implementations
- `ARCHITECTURE.md`: Detailed architecture documentation
- `Cargo.toml`: Workspace configuration

## Architecture

Extro follows a modular architecture with these key components:

1. **EXTRO CLI**: Command-line interface for user interaction
2. **EXTRO BRAIN**: Central orchestrator coordinating all components
3. **EXTRO INTENT**: Intermediate representation of extension intent
4. **AI ENGINE**: Direct code generation from intent
5. **PLATFORM ADAPTERS**: Platform-specific translation layers

For detailed architecture information, see `ARCHITECTURE.md`.

## Extending Extro

Extro is designed for extensibility. You can contribute:

1. **New Platform Adapters**:
   ```rust
   // Example of a platform adapter implementation
   pub struct NewPlatformAdapter {}
   
   impl PlatformAdapter for NewPlatformAdapter {
       // Implementation details
   }
   ```

2. **New Language Support**:
   Add language-specific code generation capabilities

3. **Custom AI Models**:
   Integrate additional AI models via Candle-compatible backends

## Next Steps

- Read the `ARCHITECTURE.md` file for detailed design information
- Explore the source code in the `src` directory
- Try creating extensions for different platforms
- Contribute to the project on GitHub

## Resources

- [Candle ML Framework](https://github.com/huggingface/candle) - Rust-native ML inference
- [llm-chain](https://github.com/sobelio/llm-chain) - Structured prompting for LLMs in Rust
- [clap](https://github.com/clap-rs/clap) - Command line argument parsing for Rust
- [Extension Development Documentation](https://github.com/jitpomi/extro/wiki) - Coming soon
