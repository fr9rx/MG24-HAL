mod generator;

use clap::Parser;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mg24-generate")]
#[command(about = "Project generator for mg24-hal (EFR32MG24)", long_about = None)]
#[command(author = "fr9rx")]
#[command(version = "0.3.0")]
struct Args {
    /// Project name to create
    name: String,

    /// Template type: blank, blink, button, i2c, dma
    #[arg(short, long, default_value = "blank")]
    template: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let project_path = PathBuf::from(&args.name);

    if project_path.exists() {
        anyhow::bail!("❌ Directory '{}' already exists", args.name);
    }

    println!("🔧 Generating mg24-hal project: {}", args.name);
    println!("   Template: {}", args.template);

    generator::create_project(&project_path, &args.template)?;

    println!("\n✅ Project created successfully!");
    println!("\n📋 Next steps:");
    println!("   cd {}", args.name);
    println!("   cargo build --release");
    println!("   cargo run --release  (with probe-rs connected)\n");

    Ok(())
}
