use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn create_project(path: &PathBuf, template: &str) -> Result<()> {
    fs::create_dir_all(&path)?;

    let project_name = path.file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid project name"))?;

    create_cargo_toml(&path, project_name)?;
    create_main_rs(&path, template)?;
    create_cargo_config(&path)?;
    create_gitignore(&path)?;

    Ok(())
}

fn create_cargo_toml(path: &PathBuf, name: &str) -> Result<()> {
    let content = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
description = "mg24-hal project"
publish = false

[dependencies]
mg24-hal = "2.0"

[[bin]]
name = "{}"
path = "src/main.rs"

[profile.release]
codegen-units = 1
lto = true
debug = true
"#,
        name, name
    );

    fs::write(path.join("Cargo.toml"), content)?;
    Ok(())
}

fn create_main_rs(path: &PathBuf, template: &str) -> Result<()> {
    let content = match template {
        "blink" => include_str!("templates/blink.rs"),
        "button" => include_str!("templates/button.rs"),
        "i2c" => include_str!("templates/i2c.rs"),
        "dma" => include_str!("templates/dma.rs"),
        _ => include_str!("templates/blank.rs"),
    };

    fs::create_dir_all(path.join("src"))?;
    fs::write(path.join("src/main.rs"), content)?;
    Ok(())
}

fn create_cargo_config(path: &PathBuf) -> Result<()> {
    let config = r#"[build]
target = "thumbv8m.main-none-eabihf"

rustflags = [
  "-C", "link-arg=-Tlink.x",
]

[target.thumbv8m.main-none-eabihf]
runner = "probe-rs run --chip EFR32MG24B220F1536IM48"

[alias]
rb = "build --release"
rr = "run --release"
size = "size --"
"#;

    fs::create_dir_all(path.join(".cargo"))?;
    fs::write(path.join(".cargo/config.toml"), config)?;
    Ok(())
}

fn create_gitignore(path: &PathBuf) -> Result<()> {
    let gitignore = r#"target/
Cargo.lock
"#;

    fs::write(path.join(".gitignore"), gitignore)?;
    Ok(())
}
