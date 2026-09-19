mod generator;
mod tui;

use clap::Parser;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mg24-generate")]
#[command(about = "Project generator for mg24-hal (EFR32MG24)", long_about = None)]
#[command(author = "fr9rx")]
#[command(version = "0.5.0")]
struct Args {
    /// Project name to create (omit or use --interactive for TUI mode)
    #[arg(value_name = "PROJECT_NAME")]
    name: Option<String>,

    /// Template type: blank, blink, button, i2c, dma
    #[arg(short, long, default_value = "blank")]
    template: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Enter interactive TUI if no name provided
    if args.name.is_none() {
        return tui::run_tui();
    }

    let project_name = args.name.unwrap();
    let project_path = PathBuf::from(&project_name);

    if project_path.exists() {
        anyhow::bail!("❌ Directory '{}' already exists", project_name);
    }

    println!("🔧 Generating mg24-hal project: {}", project_name);
    println!("   Template: {}", args.template);

    generator::create_project(&project_path, &args.template)?;

    println!("\n✅ Project created successfully!");
    println!("\n📋 Next steps:");
    println!("   cd {}", project_name);
    println!("   cargo build --release");
    println!("   cargo run --release  (with probe-rs connected)\n");

    Ok(())
}
