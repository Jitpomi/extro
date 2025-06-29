# 🧠 Extro — Write Once, Extend Everywhere

> 🛠️ Scaffold → 🔁 Transform → ⚙️ Optimize — all powered by AI.
>
> Build extensions for VSCode, Chrome, JetBrains, Blender, Unity, etc. using JS, Rust, Python, C#...

**Extro** is a universal extension framework that lets you build plugins, widgets, or add-ons for any platform — using any language you know.

✨ _Chrome, VSCode, Blender, Godot, JetBrains, Unity... all from one CLI._

> 🚀 "Write Once, Extend Everywhere" — and let AI handle the rest.

---

## ✨ What Is Extro?

Extro is a **language-agnostic, AI-powered CLI tool** for building and transforming software extensions. This is an early-stage, bootstrapped project with big ambitions.

```bash
$ extro extend

👋 Welcome to Extro! Let's create a new extension.

✅ Platform: Chrome
✅ Type: Content Script
✅ Language: TypeScript

🧠 Generating extension code...

🎉 Your extension is ready in ./my-chrome-extension

$ extro show ./my-chrome-extension png

📷 Extension visualization saved to ./my-chrome-extension/structure.png
```

### With just one command:
```bash
extro extend
```

You can scaffold fully working extensions for:

| Platform | Languages Supported | Examples |
|----------|---------------------|----------|
| Chrome | JS, TS, Python | Content scripts, background |
| VSCode | TS, Python, Rust | Developer tooling |
| Blender | Python, JS (via Extro) | 3D modeling automation |
| Godot | GDScript, C#, TS | Tools & Editor Plugins |
| JetBrains | Kotlin, JS | IDE Productivity Plugins |
| Unity | C#, JS (via Extro) | Editor & Runtime Extensions |

And many more.

## 🏁 Quick Start

```bash
# Install from crates.io
cargo install extro

# Create a new extension
extro extend
```

Or build from source:

```bash
# Clone the repository
git clone https://github.com/jitpomi/extro
cd extro

# Run directly
cargo run -- extend
```

## 🔁 Built-In Features

### 🛠️ extro extend
Create ready-to-run extensions with zero boilerplate — just answer a few prompts:
- Platform
- Extension Type
- Language
- Target App (e.g. Chrome, VSCode)

🧠 Powered by AI — DeepSeek / StarCoder generate idiomatic, working code.

### 🔁 extro transform
Convert existing extensions between:
- Platforms (e.g. Chrome → Firefox)
- Languages (e.g. JS → Python)

💡 Example: Turn your Chrome JS extension into a JetBrains Kotlin plugin.

### ⚙️ extro optimize (Coming Soon)
Analyze and enhance existing extensions:
- Remove dead code
- Add missing manifest fields
- Suggest polyfills
- Make cross-platform adaptations

## 🧠 How It Works

Extro is powered by:

- 🔁 **ExtroIntent** – A language-agnostic intermediate representation
- 🧠 **AI Engine** – Candle + llm-chain + StarCoder/DeepSeek for codegen
- 🧩 **Platform Adapters** – Pluggable targets for any software ecosystem
- 🛠️ **Direct Code Generator** – AI-first template-free scaffolding

*Build once. Run anywhere. Customize nothing.*

## 🚀 Roadmap

| Phase | Milestone |
|-------|----------|
| ✅ Q3 2025 | extro extend + initial adapters |
| 🛠️ Q4 2025 | extro transform + platform coverage |
| 🔮 Q1 2026 | extro optimize, plugin SDK |
| 🌐 Q2 2026 | Web UI + Extension Marketplace |

## 🤝 Contributing

We welcome:
- Platform Adapters (e.g. Sketch, IntelliJ, Figma)
- Language Support (Zig, Ruby, Swift, etc.)
- AI Prompt Engineers (fine-tuned StarCoder chains)
- Optimizers, Transformers, CLI features

📁 Start with CONTRIBUTING.md (coming soon)

🛠️ Rust developers especially welcome!

## ❤️ Support This Project

Extro is a bootstrapped attempt to fundamentally change how we think about plugins and extensions. We're on a mission to break down the walls between platforms and languages, making extension development accessible to everyone.

If this vision inspires you, your support makes a world of difference — even a star or $1/month keeps the mission alive and helps us dedicate more time to the project.

If this project inspires you, consider:
- 👉 ⭐ Star this repo
- 👉 [Sponsor us](SPONSORS.md)

Your support helps us bring AI to every dev, on every platform, in every language.

## 📣 Join the Mission

Want to shape the future of cross-platform extension development?
- → Apply to contribute
- → Follow @ExtroDev on Twitter
- → Discord Community (Coming Soon)

## Development

To build all components in the workspace:

```bash
cargo build
```

To run tests for all components:

```bash
cargo test
```

## License

This project is licensed under the terms specified in the LICENSE file.
