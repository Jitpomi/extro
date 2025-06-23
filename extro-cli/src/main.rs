mod commands;
// Parser macro	Tells clap to parse CLI input from std::env::args() into a struct
// Subcommand macro	Lets you define multiple commands like new, build, run

use console::style;
use crate::commands::prompts::*;
use clap::{Parser,Subcommand};
use colored::Colorize;
use console::Style;

#[derive(Subcommand)]
enum Commands {
    /// Create a new extension
    New {
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
        Commands::New { name } => {
            let ext_name = name.unwrap_or_else(|| 
                ask!(input "🛠️  What's the name of your extension?")
            );
            let ext_platform = ask!(
                select "🌍 What platform are you targeting?", 
                options
                        "🌐 Browser",
                        "💻 IDE",
                        "💬 Messaging App",
                        "🧊 3D App",
                        "🎮 Game",
                        "⚙️ Other",
            );
            let ext_type = match ext_platform.as_str() {
                "🌐 Browser" => ask!(
                select "🔧 What kind of browser extension?", 
                options
                        "🧠 Content Script",
                        "🎛️ Background Script",
                        "🎯 UI Action Button",
                        "🧰 DevTool Tab",
                        "🔧 Tool Add-on",
                ),
                "💻 IDE" => ask!(
                select "🔧 What kind of IDE extension?", 
                options
                    "🔌 Plugin",
                    "📐 Theme",
                    "🔍 Linter",
                    "🔧 Tool Add-on",
                ),
                "💬 Messaging App" => ask!(
                select "🔧 What kind of messaging app extension?", 
                options
                        "🤖 Bot",
                        "📎 Attachment Handler",
                        "📢 Notification Add-on",
                        "🔧 Tool Add-on",
                ),
                "🧊 3D App" => ask!(
                select "🔧 What kind of 3D extension?", 
                options
                        "🧱 Model Importer",
                        "🔧 Tool Add-on",
                        "🎨 Material Generator",
                        "📊 HUD Overlay",
                ),
                "🎮 Game" => ask!(
                    select "🔧 What kind of game extension?", 
                    options
                        "🧩 Mod",
                        "📊 HUD Overlay",
                        "🎮 Controller Mapper",
                ),
                "⚙️ Other" => ask!(
                    input "ℹ️  Describe the type of extension you're building:"
                ),
                _ => unreachable!(),
                };

            println!("\n✅ You chose:");
            println!("{} {}", prompt_style.apply_to("• Name:"), answer_style.apply_to(&ext_name));
            println!("{} {}", prompt_style.apply_to("• Platform:"), answer_style.apply_to(&ext_platform));
            println!("{} {}", prompt_style.apply_to("• Type:"), answer_style.apply_to(&ext_type));

            let proceed = ask!(confirm "Proceed?");

            if proceed {
                println!("✨ Scaffolding your extension...");
                // Create directories, files etc here
            } else {
                println!("❌ Canceled.");
            }

        },
    }
}
