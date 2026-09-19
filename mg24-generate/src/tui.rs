use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout, SelectView, EditView, TextView};
use cursive::Cursive;
use std::path::PathBuf;
use anyhow::Result;

use crate::generator;

pub fn run_tui() -> Result<()> {
    let mut siv = Cursive::try_new_with_backend(cursive::backends::crossterm::Backend::init())?;

    show_welcome(&mut siv);

    while siv.is_running() {
        siv.step();
    }

    Ok(())
}

fn show_welcome(siv: &mut Cursive) {
    let welcome_text = "\
╔════════════════════════════════════════╗
║  mg24-generate v0.2.0                  ║
║  Interactive Project Generator         ║
║  for mg24-hal (EFR32MG24)              ║
╚════════════════════════════════════════╝

Create new mg24-hal projects with templates.
";

    siv.add_layer(Dialog::new()
        .title("Welcome")
        .content(TextView::new(welcome_text))
        .button("Get Started", |s| {
            s.pop_layer();
            show_project_name_dialog(s);
        })
        .button("Quit", |s| s.quit()));
}

fn show_project_name_dialog(siv: &mut Cursive) {
    let dialog = Dialog::new()
        .title("Project Name")
        .content(
            LinearLayout::vertical()
                .child(TextView::new("Enter project name:"))
                .child(EditView::new()
                    .with_name("project_name")
                    .fixed_width(30))
        )
        .button("Next", |s| {
            let name = s.call_on_name("project_name", |v: &mut EditView| {
                v.get_content()
            });

            if let Some(name) = name {
                if !name.is_empty() {
                    let name_copy = name.to_string();
                    s.pop_layer();
                    show_template_dialog(s, name_copy);
                }
            }
        })
        .button("Back", |s| {
            s.pop_layer();
            show_welcome(s);
        });

    siv.add_layer(dialog);
}

fn show_template_dialog(siv: &mut Cursive, project_name: String) {
    let template_select = SelectView::new()
        .item("blank (empty project)", "blank")
        .item("blink (LED blinking)", "blink")
        .item("button (button input)", "button")
        .item("i2c (I2C communication)", "i2c")
        .item("dma (DMA transfer)", "dma")
        .with_name("template_select");

    let dialog = Dialog::new()
        .title("Select Template")
        .content(template_select)
        .button("Create", {
            let name = project_name.clone();
            move |s| {
                let template = s.call_on_name("template_select", |v: &mut SelectView<&str>| {
                    v.selection().map(|t| t.to_string())
                });

                if let Some(Some(template)) = template {
                    let project_path = PathBuf::from(&name);

                    if project_path.exists() {
                        s.add_layer(Dialog::new()
                            .title("Error")
                            .content(TextView::new(
                                format!("Directory '{}' already exists!", name)
                            ))
                            .button("OK", |s| {
                                s.pop_layer();
                            }));
                    } else {
                        match generator::create_project(&project_path, &template) {
                            Ok(()) => {
                                s.pop_layer();
                                show_success(s, &name, &template);
                            }
                            Err(e) => {
                                s.add_layer(Dialog::new()
                                    .title("Error")
                                    .content(TextView::new(format!("Error: {}", e)))
                                    .button("OK", |s| {
                                        s.pop_layer();
                                    }));
                            }
                        }
                    }
                }
            }
        })
        .button("Back", {
            let name = project_name.clone();
            move |s| {
                s.pop_layer();
                show_project_name_dialog(s);
            }
        });

    siv.add_layer(dialog);
}

fn show_success(siv: &mut Cursive, name: &str, template: &str) {
    let success_text = format!(
        "✅ Project Created Successfully!\n\n\
         Project:  {}\n\
         Template: {}\n\n\
         Next steps:\n\
         • cd {}\n\
         • cargo build --release\n\
         • cargo run --release\n\n\
         (with probe-rs connected)",
        name, template, name
    );

    siv.add_layer(Dialog::new()
        .title("Success")
        .content(TextView::new(success_text))
        .button("Exit", |s| s.quit()));
}
