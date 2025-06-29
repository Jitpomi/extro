mod commands;
// Parser macro	Tells clap to parse CLI input from std::env::args() into a struct
// Subcommand macro	Lets you define multiple commands like new, build, run

use console::style;
use crate::commands::prompts::*;
use clap::{Parser,Subcommand};
use colored::Colorize;
use console::Style;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct ExtroManifest {
    name: String,
    platform: String,
    #[serde(rename = "type")]
    type_: String,
    language: String,
    entrypoint: String,
    features: HashMap<String, bool>,
}

impl ExtroManifest {
    fn new(name: String, platform: String, type_: String, language: String) -> Self {
        // Normalize platform and type names for the manifest
        let platform_normalized = Self::normalize_platform(&platform);
        let type_normalized = Self::normalize_type(&type_);
        let language_normalized = Self::normalize_language(&language);
        
        // Infer entrypoint based on language
        let entrypoint = Self::infer_entrypoint(&language_normalized, &platform_normalized, &type_normalized);
        
        // Infer features based on type
        let features = Self::infer_features(&platform_normalized, &type_normalized);
        
        ExtroManifest {
            name,
            platform: platform_normalized,
            type_: type_normalized,
            language: language_normalized,
            entrypoint,
            features,
        }
    }
    
    fn normalize_platform(platform: &str) -> String {
        // Remove emoji and trim
        let platform = platform.trim();
        let platform = platform.split_whitespace().last().unwrap_or(platform);
        
        match platform {
            "Browser" => "browser".to_string(),
            "IDE/Code Editor" => "ide".to_string(),
            "DevOps Tools" => "devops".to_string(),
            "3D/Graphics Apps" => "graphics".to_string(),
            "Game Engines" => "game".to_string(),
            "Data/Analytics" => "data".to_string(),
            "AI/ML Platforms" => "ai".to_string(),
            "Mobile Platforms" => "mobile".to_string(),
            "Desktop Apps" => "desktop".to_string(),
            "Cloud Platforms" => "cloud".to_string(),
            _ => platform.to_lowercase(),
        }
    }
    
    fn normalize_type(type_: &str) -> String {
        // Remove emoji and convert to kebab-case
        let type_ = type_.trim();
        let type_ = type_.split_whitespace().collect::<Vec<&str>>().join("-").to_lowercase();
        
        match type_.as_str() {
            "content-script" => "content-script".to_string(),
            "background-script" => "background-script".to_string(),
            "ui-action-button" => "ui-action".to_string(),
            "devtool-tab" => "devtool".to_string(),
            "tool-add-on" => "tool".to_string(),
            "plugin" => "plugin".to_string(),
            "theme" => "theme".to_string(),
            "linter" => "linter".to_string(),
            "bot" => "bot".to_string(),
            "model-importer" => "importer".to_string(),
            "material-generator" => "generator".to_string(),
            "hud-overlay" => "overlay".to_string(),
            "mod" => "mod".to_string(),
            "controller-mapper" => "controller".to_string(),
            _ => type_,
        }
    }
    
    fn normalize_language(language: &str) -> String {
        match language {
            "JavaScript" => "js".to_string(),
            "TypeScript" => "ts".to_string(),
            "Python" => "py".to_string(),
            "Rust" => "rs".to_string(),
            "Go" => "go".to_string(),
            "C#" => "cs".to_string(),
            "Java" => "java".to_string(),
            "Kotlin" => "kt".to_string(),
            "Swift" => "swift".to_string(),
            "Dart" => "dart".to_string(),
            "Zig" => "zig".to_string(),
            "Ruby" => "rb".to_string(),
            "C++" => "cpp".to_string(),
            "GDScript" => "gd".to_string(),
            "Lua" => "lua".to_string(),
            "PHP" => "php".to_string(),
            "MEL" => "mel".to_string(),
            "MAXScript" => "ms".to_string(),
            "VEX" => "vex".to_string(),
            "HScript" => "hscript".to_string(),
            "GLSL" => "glsl".to_string(),
            "HLSL" => "hlsl".to_string(),
            "OSL" => "osl".to_string(),
            _ => language.to_lowercase(),
        }
    }
    
    fn infer_entrypoint(language: &str, platform: &str, type_: &str) -> String {
        match language {
            "js" => match platform {
                "browser" => match type_ {
                    "content-script" => "src/content.js".to_string(),
                    "background-script" => "src/background.js".to_string(),
                    "ui-action" => "src/popup.js".to_string(),
                    "devtool" => "src/devtool.js".to_string(),
                    _ => "src/index.js".to_string(),
                },
                _ => "src/index.js".to_string(),
            },
            "ts" => match platform {
                "browser" => match type_ {
                    "content-script" => "src/content.ts".to_string(),
                    "background-script" => "src/background.ts".to_string(),
                    "ui-action" => "src/popup.ts".to_string(),
                    "devtool" => "src/devtool.ts".to_string(),
                    _ => "src/index.ts".to_string(),
                },
                _ => "src/index.ts".to_string(),
            },
            "py" => match platform {
                "graphics" => "src/__init__.py".to_string(),
                _ => "src/main.py".to_string(),
            },
            "rs" => "src/lib.rs".to_string(),
            "go" => "cmd/main.go".to_string(),
            "cs" => match platform {
                "game" => "Assets/Scripts/Main.cs".to_string(),
                _ => "src/Program.cs".to_string(),
            },
            "java" => "src/main/java/com/extro/Main.java".to_string(),
            "kt" => "src/main/kotlin/com/extro/Main.kt".to_string(),
            "swift" => "Sources/Main.swift".to_string(),
            "dart" => "lib/main.dart".to_string(),
            "zig" => "src/main.zig".to_string(),
            "rb" => "lib/main.rb".to_string(),
            "cpp" => match platform {
                "graphics" => "src/plugin.cpp".to_string(),
                "game" => "src/mod.cpp".to_string(),
                _ => "src/main.cpp".to_string(),
            },
            "gd" => "addons/extro/plugin.gd".to_string(),
            "lua" => "src/main.lua".to_string(),
            "php" => "src/index.php".to_string(),
            "mel" => "scripts/main.mel".to_string(),
            "ms" => "scripts/main.ms".to_string(),
            "vex" => "vex/main.vfl".to_string(),
            "hscript" => "scripts/main.hscript".to_string(),
            "glsl" => "shaders/main.glsl".to_string(),
            "hlsl" => "shaders/main.hlsl".to_string(),
            "osl" => "shaders/main.osl".to_string(),
            _ => format!("src/main.{}", language),
        }
    }
    
    fn infer_features(platform: &str, type_: &str) -> HashMap<String, bool> {
        let mut features = HashMap::new();
        
        // Default features based on platform
        match platform {
            "browser" => {
                features.insert("dom".to_string(), type_ == "content-script");
                features.insert("network".to_string(), type_ == "background-script");
                features.insert("ui".to_string(), type_ == "ui-action");
                features.insert("storage".to_string(), true);
            },
            "ide" => {
                features.insert("ui".to_string(), true);
                features.insert("workspace".to_string(), true);
            },
            "graphics" => {
                features.insert("ui".to_string(), true);
                features.insert("3d".to_string(), true);
            },
            "game" => {
                features.insert("ui".to_string(), type_ == "overlay");
                features.insert("input".to_string(), type_ == "controller");
                features.insert("assets".to_string(), true);
            },
            _ => {
                features.insert("ui".to_string(), false);
                features.insert("network".to_string(), false);
            },
        }
        
        features
    }
    
    fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::create_dir_all(Path::new(path).parent().unwrap())?;
        fs::write(path, json)
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new extension
    Extend {
        /// The name of the extension (optional will be prompted if not provided)
        name: Option<String>,
    },
}
#[derive(Parser)]
#[command( name = "extro", about = "Build once, run anywhere!", version = "0.1.0" )]
struct Cli {
    // #[command(...)]	Tells clap this field is a subcommand
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    println!(
        "{}",
        r#"

⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣿⣦⡀⠀⠀⠀⠰⣶⡄⠀⣰⡶⠂⠶⠶⣶⡶⠶⠶⢰⣶⠶⠶⣶⣤⠀⢀⣴⡶⠶⣶⣦⡀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣿⣿⣄⡀⠀⠀⠙⣿⣼⡟⠁⠀⠀⠀⢸⡇⠀⠀⢸⣿⠀⠀⢸⣿⢇⣿⡏⠀⠀⠀⢻⣿⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⠏⠀⠀⠀⣠⣾⢿⣷⡀⠀⠀⠀⢸⡇⠀⠀⢸⣿⠿⢿⣟⡋⠘⣿⣇⠀⠀⠀⣸⡿⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿⠟⠁⠀⠀⠀⣴⣿⠃⠀⠹⣷⡄⠀⠀⢸⡇⠀⠀⠸⣿⠀⠀⢻⣷⠄⠈⠻⣷⣶⡾⠟⠁⠀⠀⠀⠀⠀⠀⠀

             EXTEND • TRANSFORM • OPTIMIZE

"#
        .bright_cyan()
    );
    let prompt_style = Style::new().cyan().bold();
    let answer_style = Style::new().green().bold().italic();
    let cli = Cli::parse();
    match cli.command {
        Commands::Extend { name } => {
            let ext_name = name.unwrap_or_else(|| 
                ask!(input "🛠️  What's the name of your extension?")
            );
            let ext_platform = ask!(
                select "🌍 What platform are you targeting?", 
                options
                        "🌐 Browser",
                        "💻 IDE/Code Editor",
                        "🧰 DevOps Tools",
                        "🧊 3D/Graphics Apps",
                        "🎮 Game Engines",
                        "📊 Data/Analytics",
                        "🤖 AI/ML Platforms",
                        "📱 Mobile Platforms",
                        "🖥️ Desktop Apps",
                        "☁️ Cloud Platforms",
                        "⚙️ Other",
            );
            
            // Ask for specific application within the selected platform
            let specific_app = match ext_platform.as_str() {
                "🌐 Browser" => ask!(
                    select "🔍 Which browser are you targeting?", 
                    options
                            "Chrome",
                            "Firefox",
                            "Safari",
                            "Edge",
                            "Opera",
                            "Brave",
                            "Other",
                ),
                "💻 IDE/Code Editor" => ask!(
                    select "🔍 Which IDE or code editor are you targeting?", 
                    options
                            "VS Code",
                            "JetBrains (IntelliJ, WebStorm, etc.)",
                            "Visual Studio",
                            "Neovim/Vim",
                            "Emacs",
                            "Sublime Text",
                            "Atom",
                            "Eclipse",
                            "Xcode",
                            "Android Studio",
                            "Other",
                ),
                "🧰 DevOps Tools" => ask!(
                    select "🔍 Which DevOps tool are you targeting?", 
                    options
                            "GitHub Actions",
                            "GitLab CI/CD",
                            "Jenkins",
                            "Terraform",
                            "Docker",
                            "Kubernetes",
                            "Ansible",
                            "CircleCI",
                            "Travis CI",
                            "ArgoCD",
                            "Other",
                ),
                "🧊 3D/Graphics Apps" => ask!(
                    select "🔍 Which 3D or graphics application are you targeting?", 
                    options
                            "Blender",
                            "Maya",
                            "3ds Max",
                            "Houdini",
                            "Cinema 4D",
                            "ZBrush",
                            "Substance Designer/Painter",
                            "Photoshop",
                            "Illustrator",
                            "After Effects",
                            "Nuke",
                            "Fusion",
                            "Other",
                ),
                "🎮 Game Engines" => ask!(
                    select "🔍 Which game engine are you targeting?", 
                    options
                            "Unity",
                            "Unreal Engine",
                            "Godot",
                            "CryEngine",
                            "Lumberyard",
                            "GameMaker Studio",
                            "Construct",
                            "Cocos",
                            "Defold",
                            "PlayCanvas",
                            "Other",
                ),
                "📊 Data/Analytics" => ask!(
                    select "🔍 Which data or analytics platform are you targeting?", 
                    options
                            "Jupyter",
                            "Tableau",
                            "Power BI",
                            "Grafana",
                            "Kibana",
                            "Looker",
                            "Metabase",
                            "Redash",
                            "Superset",
                            "Streamlit",
                            "Other",
                ),
                "🤖 AI/ML Platforms" => ask!(
                    select "🔍 Which AI/ML platform are you targeting?", 
                    options
                            "TensorFlow",
                            "PyTorch",
                            "Hugging Face",
                            "LangChain",
                            "MLflow",
                            "Weights & Biases",
                            "Gradio",
                            "Label Studio",
                            "Roboflow",
                            "Vertex AI",
                            "Other",
                ),
                "📱 Mobile Platforms" => ask!(
                    select "🔍 Which mobile platform are you targeting?", 
                    options
                            "Android",
                            "iOS",
                            "React Native",
                            "Flutter",
                            "Xamarin",
                            "Cordova/PhoneGap",
                            "Ionic",
                            "Capacitor",
                            "NativeScript",
                            "Other",
                ),
                "🖥️ Desktop Apps" => ask!(
                    select "🔍 Which desktop application framework are you targeting?", 
                    options
                            "Electron",
                            "Tauri",
                            "Qt",
                            "GTK",
                            "WPF (.NET)",
                            "Windows Forms",
                            "SwiftUI",
                            "JavaFX",
                            "wxWidgets",
                            "Other",
                ),
                "☁️ Cloud Platforms" => ask!(
                    select "🔍 Which cloud platform are you targeting?", 
                    options
                            "AWS",
                            "Azure",
                            "Google Cloud",
                            "Cloudflare",
                            "Vercel",
                            "Netlify",
                            "Heroku",
                            "DigitalOcean",
                            "Salesforce",
                            "Other",
                ),
                "⚙️ Other" => ask!(
                    input "🔍 What specific platform are you targeting?"
                ),
                _ => unreachable!(),
            };

            let ext_type = match ext_platform.as_str() {
                "🌐 Browser" => {
                    let browser_name = specific_app.clone();
                    ask!(
                        select format!("🔧 What kind of {} extension?", browser_name).as_str(), 
                        options
                                "🧠 Content Script",
                                "🎛️ Background Script",
                                "🎯 UI Action Button",
                                "🧰 DevTool Tab",
                                "🔧 Tool Add-on",
                    )
                },
                "💻 IDE/Code Editor" => {
                    let ide_name = specific_app.clone();
                    ask!(
                        select format!("🔧 What kind of {} extension?", ide_name).as_str(), 
                        options
                            "🔌 Plugin",
                            "📐 Theme",
                            "🔍 Linter",
                            "🔧 Tool Add-on",
                    )
                },
                "🧰 DevOps Tools" => {
                    let tool_name = specific_app.clone();
                    ask!(
                        select format!("🔧 What kind of {} extension?", tool_name).as_str(), 
                        options
                                "🔩 Action",
                                "📈 Workflow",
                                "🔧 Tool Add-on",
                    )
                },
                "🧊 3D/Graphics Apps" => {
                    let app_name = specific_app.clone();
                    ask!(
                        select format!("🔧 What kind of {} extension?", app_name).as_str(), 
                        options
                                "🧱 Model Importer",
                                "🔧 Tool Add-on",
                                "🎨 Material Generator",
                                "📊 HUD Overlay",
                    )
                },
                "🎮 Game Engines" => {
                    let engine_name = specific_app.clone();
                    ask!(
                        select format!("🔧 What kind of {} extension?", engine_name).as_str(), 
                        options
                                "🧩 Mod",
                                "📊 HUD Overlay",
                                "🎮 Controller Mapper",
                    )
                },
                "📊 Data/Analytics" => {
                    let platform_name = specific_app.clone();
                    ask!(
                        select format!("🔧 What kind of {} extension?", platform_name).as_str(), 
                        options
                                "📈 Dashboard",
                                "📊 Data Connector",
                                "🔧 Tool Add-on",
                    )
                },
                "🤖 AI/ML Platforms" => {
                    let platform_name = specific_app.clone();
                    ask!(
                        select format!("🔧 What kind of {} extension?", platform_name).as_str(), 
                        options
                                "🤖 Model",
                                "📊 Data Connector",
                                "🔧 Tool Add-on",
                    )
                },
                "📱 Mobile Platforms" => {
                    let platform_name = specific_app.clone();
                    ask!(
                        select format!("🔧 What kind of {} extension?", platform_name).as_str(), 
                        options
                                "📈 Plugin",
                                "📊 SDK",
                                "🔧 Tool Add-on",
                    )
                },
                "🖥️ Desktop Apps" => {
                    let framework_name = specific_app.clone();
                    ask!(
                        select format!("🔧 What kind of {} extension?", framework_name).as_str(), 
                        options
                                "📈 Plugin",
                                "📊 SDK",
                                "🔧 Tool Add-on",
                    )
                },
                "☁️ Cloud Platforms" => {
                    let platform_name = specific_app.clone();
                    ask!(
                        select format!("🔧 What kind of {} extension?", platform_name).as_str(), 
                        options
                                "📈 Plugin",
                                "📊 SDK",
                                "🔧 Tool Add-on",
                    )
                },
                "⚙️ Other" => ask!(
                    input "ℹ️  Describe the type of extension you're building:"
                ),
                _ => unreachable!(),
            };

            println!("\n✅ You chose:");
            println!("{} {}", prompt_style.apply_to("• Name:"), answer_style.apply_to(&ext_name));
            println!("{} {}", prompt_style.apply_to("• Platform:"), answer_style.apply_to(&ext_platform));
            println!("{} {}", prompt_style.apply_to("• Specific App:"), answer_style.apply_to(&specific_app));
            println!("{} {}", prompt_style.apply_to("• Type:"), answer_style.apply_to(&ext_type));

            // Determine default language based on platform and specific app
            let default_language_index = match ext_platform.as_str() {
                "🌐 Browser" => 0, // JavaScript for browsers
                "💻 IDE/Code Editor" => {
                    match specific_app.as_str() {
                        "VS Code" => 1, // TypeScript for VS Code
                        "JetBrains (IntelliJ, WebStorm, etc.)" => 5, // Java for JetBrains
                        "Visual Studio" => 5, // C# for Visual Studio
                        _ => 0, // JavaScript as fallback
                    }
                },
                "🧰 DevOps Tools" => {
                    match specific_app.as_str() {
                        "GitHub Actions" => 1, // TypeScript for GitHub Actions
                        "GitLab CI/CD" => 2, // Python for GitLab CI/CD
                        _ => 0, // JavaScript as fallback
                    }
                },
                "🧊 3D/Graphics Apps" => {
                    match specific_app.as_str() {
                        "Blender" => 2, // Python for Blender
                        "Maya" => 2, // Python for Maya
                        "3ds Max" => 2, // Python for 3ds Max
                        "Houdini" => 2, // Python for Houdini
                        "Cinema 4D" => 2, // Python for Cinema 4D
                        "Substance Designer/Painter" => 2, // Python for Substance Designer/Painter
                        "Photoshop" => 1, // TypeScript for Photoshop
                        "Illustrator" => 1, // TypeScript for Illustrator
                        "After Effects" => 1, // TypeScript for After Effects
                        _ => 2, // Python as fallback for 3D apps
                    }
                },
                "🎮 Game Engines" => {
                    match specific_app.as_str() {
                        "Unity" => 5, // C# for Unity
                        "Unreal Engine" => 12, // C++ for Unreal
                        "Godot" => 13, // GDScript for Godot
                        _ => 5, // C# as fallback for game development
                    }
                },
                "📊 Data/Analytics" => {
                    match specific_app.as_str() {
                        "Jupyter" => 2, // Python for Jupyter
                        "Tableau" => 1, // TypeScript for Tableau
                        "Power BI" => 1, // TypeScript for Power BI
                        _ => 2, // Python as fallback for data/analytics
                    }
                },
                "🤖 AI/ML Platforms" => {
                    match specific_app.as_str() {
                        "TensorFlow" => 2, // Python for TensorFlow
                        "PyTorch" => 2, // Python for PyTorch
                        "Hugging Face" => 2, // Python for Hugging Face
                        _ => 2, // Python as fallback for AI/ML
                    }
                },
                "📱 Mobile Platforms" => {
                    match specific_app.as_str() {
                        "Android" => 5, // Java for Android
                        "iOS" => 9, // Swift for iOS
                        "React Native" => 1, // TypeScript for React Native
                        "Flutter" => 4, // Dart for Flutter
                        _ => 5, // Java as fallback for mobile
                    }
                },
                "🖥️ Desktop Apps" => {
                    match specific_app.as_str() {
                        "Electron" => 1, // TypeScript for Electron
                        "Tauri" => 1, // TypeScript for Tauri
                        "Qt" => 5, // C++ for Qt
                        _ => 1, // TypeScript as fallback for desktop
                    }
                },
                "☁️ Cloud Platforms" => {
                    match specific_app.as_str() {
                        "AWS" => 1, // TypeScript for AWS
                        "Azure" => 5, // C# for Azure
                        "Google Cloud" => 1, // TypeScript for Google Cloud
                        _ => 1, // TypeScript as fallback for cloud
                    }
                },
                _ => 0, // JavaScript as general fallback
            };

            // Define language options
            let language_options = [
                "JavaScript",
                "TypeScript",
                "Python",
                "Rust",
                "Go",
                "C#",
                "Java",
                "Kotlin",
                "Swift",
                "Dart",
                "Zig",
                "Ruby",
                "C++",
                "GDScript",
                "Lua",
                "PHP",
                "MEL",
                "MAXScript",
                "VEX",
                "HScript",
                "GLSL",
                "HLSL",
                "OSL",
                "Blueprints",
                "VisualScript",
                "Other",
            ];
            
            // Ask for preferred programming language using Select directly to set default
            let programming_language = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("💻 What programming language would you like to use?")
                .items(&language_options)
                .default(default_language_index)
                .interact()
                .unwrap();
            
            let selected_language = language_options[programming_language];
            
            // Handle "Other" language selection
            let final_language = if selected_language == "Other" {
                ask!(input "📝 Please specify your preferred programming language:")
            } else {
                selected_language.to_string()
            };
            
            println!("{} {}", prompt_style.apply_to("• Language:"), answer_style.apply_to(&final_language));

            let proceed = ask!(confirm "Proceed?");

            if proceed {
                println!("✨ Scaffolding your extension...");
                
                // Create the manifest
                let manifest = ExtroManifest::new(
                    ext_name.clone(),
                    ext_platform.clone(),
                    ext_type.clone(),
                    final_language.clone(),
                );
                
                // Create project directory
                let project_dir = format!("./{}", ext_name);
                fs::create_dir_all(&project_dir).expect("Failed to create project directory");
                
                // Save manifest
                let manifest_path = format!("{}/extro.json", project_dir);
                manifest.save_to_file(&manifest_path).expect("Failed to save manifest");
                
                println!("📄 Created manifest at {}", manifest_path);
                println!("📁 Project structure:");
                println!("  📂 {}/", ext_name);
                println!("  ├─ 📄 extro.json");
                println!("  └─ 📄 {}", manifest.entrypoint);
                
                // TODO: Generate project files based on manifest
                
                println!("\n✅ Extension scaffolded successfully!");
                println!("🚀 Run the following commands to get started:");
                println!("  cd {}", ext_name);
                println!("  # Start developing your extension");
            } else {
                println!("❌ Canceled.");
            }

        },
    }
}
