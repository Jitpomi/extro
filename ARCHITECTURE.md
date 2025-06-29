# Extro Architecture: Write Once, Extend Everywhere

## Vision

Extro is a universal extension framework that empowers anyone to build plugins, widgets, or add-ons for any platform (VSCode, Blender, Chrome, JetBrains, Godot, Figma, etc.) using any language they know (JS, Python, C++, GDScript, etc.).

> "Write Once, Extend Everywhere"



## Key Features

### 🛠️ extro extend
- Creates a ready-to-go extension project in any language for any platform with zero boilerplate work
- User answers prompts (app, type, language, etc.)
- Extro generates a working skeleton (manifest.json, index.js, plugin.xml, etc.)
- Powered by AI (DeepSeek/StarCoder/etc.)

### 🔁 extro transform
- Converts existing extensions from one platform or language to another
- Examples:
  - VSCode plugin → JetBrains plugin
  - Chrome extension in JS → Firefox add-on in TS
  - Godot GDScript tool → Unity C# Editor script
- Powered by:
  - An intermediate representation (Extro Intent Format / ExtroManifest)
  - AI-based translators (e.g., DeepSeek, StarCoder)

### ⚙️ extro optimize (WIP)
- Introspects & suggests improvements to existing extensions:
  - Removes unused code
  - Adds missing manifest fields
  - Suggests polyfills or cross-platform adaptations
- Powered by code analysis + LLMs

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           EXTRO ARCHITECTURE                            │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                ┌──────────────────┬┴┬──────────────────┐
                │                  │ │                  │
┌───────────────▼───────────────┐  │ │  ┌───────────────▼───────────────┐
│         EXTRO CLI (Rust)      │  │ │  │         EXTRO BRAIN           │
│                               │  │ │  │                               │
│ ┌───────────┐   ┌───────────┐ │  │ │  │ ┌───────────┐   ┌───────────┐ │
│ │  extro    │   │  extro    │ │  │ │  │ │ Platform  │   │ Language  │ │
│ │  extend   │◄──┤ transform │ │  │ │  │ │ Registry  │   │ Registry  │ │
│ └───────────┘   └───────────┘ │  │ │  │ └───────────┘   └───────────┘ │
│        │             │        │  │ │  │        │             │        │
│ ┌───────────┐   ┌───────────┐ │  │ │  │ ┌───────────┐   ┌───────────┐ │
│ │  extro    │   │  Command  │ │  │ │  │ │  Schema   │   │  Plugin   │ │
│ │ optimize  │   │  Router   │ │  │ │  │ │ Validator │   │  Loader   │ │
│ └───────────┘   └───────────┘ │  │ │  │ └───────────┘   └───────────┘ │
└───────────────────────────────┘  │ │  └───────────────────────────────┘
                │                  │ │                  │
                └──────────────────┘ └──────────────────┘
                                    │
                ┌──────────────────┬┴┬──────────────────┐
                │                  │ │                  │
┌───────────────▼───────────────┐  │ │  ┌───────────────▼───────────────┐
│    EXTRO INTENT (IR)          │  │ │  │      AI ENGINE (Rust)         │
│                               │  │ │  │                               │
│ ┌───────────┐   ┌───────────┐ │  │ │  │ ┌───────────┐   ┌───────────┐ │
│ │ Platform  │   │ Extension │ │  │ │  │ │  Candle   │   │ llm-chain │ │
│ │ Adapters  │   │  Schema   │ │  │ │  │ │  Engine   │   │   API     │ │
│ └───────────┘   └───────────┘ │  │ │  │ └───────────┘   └───────────┘ │
│        │             │        │  │ │  │        │             │        │
│ ┌───────────┐   ┌───────────┐ │  │ │  │ ┌───────────┐   ┌───────────┐ │
│ │ Extension │   │ Validator │ │  │ │  │ │ DeepSeek/ │   │  Direct   │ │
│ │  Parser   │   │   Rules   │ │  │ │  │ │ StarCoder │   │ Generator │ │
│ └───────────┘   └───────────┘ │  │ │  │ └───────────┘   └───────────┘ │
└───────────────────────────────┘  │ │  └───────────────────────────────┘
                │                  │ │                  │
                └──────────────────┘ └──────────────────┘
                                    │
┌─────────────────────────────────────────────────────────────────────────┐
│                        PLATFORM ADAPTERS                                │
│                                                                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │  VSCode  │  │ JetBrains│  │  Chrome  │  │  Blender │  │  Godot   │  │
│  │ Adapter  │  │ Adapter  │  │ Adapter  │  │ Adapter  │  │ Adapter  │  │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘  └──────────┘  │
│                                                                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │  Figma   │  │  Unity   │  │ Firefox  │  │   AWS    │  │ Platform │  │
│  │ Adapter  │  │ Adapter  │  │ Adapter  │  │ Adapter  │  │ Adapter  │  │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘  └──────────┘  │
└─────────────────────────────────────────────────────────────────────────┘
```

## Component Descriptions

### 1. EXTRO CLI (Rust)
- **extro extend**: Interactive scaffolding command that prompts users for platform, extension type, and language preferences, then generates a working extension skeleton
- **extro transform**: Converts existing extensions between platforms/languages using the ExtroIntent as an intermediate representation
- **extro optimize**: Analyzes and suggests improvements to existing extensions
- **Command Router**: Handles CLI argument parsing and routes to appropriate commands

### 2. EXTRO BRAIN (Orchestrator)
- **Central Orchestrator**: Coordinates the flow between CLI, ExtroIntent, and Platform Adapters
- **Platform Registry**: Database of supported platforms and their requirements
- **Language Registry**: Database of supported languages and their capabilities
- **Schema Validator**: Validates extension configurations against platform requirements
- **Plugin Loader**: Dynamically loads platform-specific adapters and extensions

### 3. EXTRO INTENT (IR - Intermediate Representation)
- **Platform Adapters**: Translation layers between platform-specific formats and Extro's IR
- **Extension Schema**: Universal schema defining extension capabilities and requirements
- **Extension Parser**: Converts existing extensions to the Extro IR format
- **Validator Rules**: Rules for validating extensions against platform requirements

### 4. AI ENGINE (Rust)
- **Candle Engine**: Rust-native ML inference for code generation and transformation
- **llm-chain API**: Structured prompting and chain-of-thought reasoning
- **DeepSeek/StarCoder**: LLM models for code generation and transformation
- **Direct Code Generator**: Generates complete, idiomatic code directly from ExtroIntent without templates

### 5. PLATFORM ADAPTERS
- Platform-specific modules that handle:
  - Manifest translation (platform ↔ Extro IR)
  - Platform requirements and constraints
  - Platform-specific validation rules
  - Build/packaging instructions

## Data Flow

### 1. extro extend
- User input → CLI → Platform/Language Registry → AI Engine → Direct Code Generator → Generated Extension

### 2. extro transform
- Source Extension → Extension Parser → ExtroIntent → AI Engine → Direct Code Generator → Generated Extension

### 3. extro optimize
- Existing Extension → Extension Parser → Validator Rules → AI Engine → Improvement Suggestions

## Key Technical Components

### 1. Extro Intent Format (ExtroIntent)
- Universal schema for describing extension capabilities
- Platform-agnostic representation of extension functionality
- Bidirectional translation between platform-specific formats

### 2. AI-Powered Code Generation
- Rust-native inference with Candle
- Structured prompting with llm-chain
- AI-powered direct code generation

### 3. Platform Adapters
- Pluggable architecture for adding new platforms
- Each adapter contains platform-specific knowledge:
  - Manifest format
  - Required files
  - API conventions
  - Build/packaging instructions

### 4. Language Support
- Language-specific code generation capabilities
- Cross-language translation capabilities
- Idiomatic code generation for each language

## Key Philosophy
- Language-agnostic
- Platform-unifying
- Zero friction for non-native devs
- Powered by AI (scaffolding, translation, optimization)

## Competitive Edge
Unlike existing tools (like Plasmo, Yeoman, or Cookiecutter):
- Not just code scaffolding
- Translates intent across platforms and languages
- Cross-language + cross-platform + AI-powered
- Rust-native precision using Candle, llm-chain, and procedural macros

## Extensibility
- Extro supports plugin-based adapters and future `extro plugin` commands
- Third-party contributors can add new:
  - Platforms (e.g., IntelliJ, Sketch)
  - Languages (e.g., Zig, Swift)
  - AI Models (via Candle-compatible backends)
- SDK will be provided for extending Extro's capabilities

## AI-First Approach
- **DIRECT GENERATION**: Code is generated directly from ExtroIntent using AI inference (e.g., prompt-to-code with DeepSeek)
- This unlocks extension creation **for any platform/language without predefined patterns**
- AI models can learn from existing extensions to generate new ones
- Continuous improvement through feedback loops

## Roadmap
- **Q3 2025**: Initial release with `extro extend` and core platforms
- **Q4 2025**: `extro transform` between major platforms
- **Q1 2026**: `extro optimize` and plugin SDK
- **Q2 2026**: Community adapters and extension marketplace
