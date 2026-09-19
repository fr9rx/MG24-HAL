use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, List, ListItem, Widget},
    Terminal, Frame, buffer::Buffer,
};
use std::io;
use std::path::PathBuf;

use crate::generator;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Screen {
    Welcome,
    ProjectName,
    TemplateSelect,
    Success,
}

pub struct App {
    screen: Screen,
    project_name: String,
    templates: Vec<&'static str>,
    selected_template: usize,
    message: String,
}

impl App {
    fn new() -> Self {
        Self {
            screen: Screen::Welcome,
            project_name: String::new(),
            templates: vec!["blank", "blink", "button", "i2c", "dma"],
            selected_template: 0,
            message: String::new(),
        }
    }
}

pub fn run_tui() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    result
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| render_frame(f, &app))?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if handle_key(&mut app, key) {
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

fn handle_key(app: &mut App, key: KeyEvent) -> bool {
    match app.screen {
        Screen::Welcome => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => true,
            KeyCode::Enter => {
                app.screen = Screen::ProjectName;
                false
            }
            _ => false,
        },
        Screen::ProjectName => match key.code {
            KeyCode::Char(c) => {
                app.project_name.push(c);
                false
            }
            KeyCode::Backspace => {
                app.project_name.pop();
                false
            }
            KeyCode::Enter => {
                if !app.project_name.is_empty() {
                    app.screen = Screen::TemplateSelect;
                }
                false
            }
            KeyCode::Esc => {
                app.screen = Screen::Welcome;
                app.project_name.clear();
                false
            }
            _ => false,
        },
        Screen::TemplateSelect => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if app.selected_template > 0 {
                    app.selected_template -= 1;
                }
                false
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.selected_template < app.templates.len() - 1 {
                    app.selected_template += 1;
                }
                false
            }
            KeyCode::Enter => {
                let template = app.templates[app.selected_template];
                let project_path = PathBuf::from(&app.project_name);

                if project_path.exists() {
                    app.message = format!("ERROR: Directory '{}' already exists!", app.project_name);
                } else {
                    match generator::create_project(&project_path, template) {
                        Ok(()) => {
                            app.message = format!(
                                "✓ Project '{}' created with '{}' template\n\ncd {}  &&  cargo build --release",
                                app.project_name, template, app.project_name
                            );
                        }
                        Err(e) => {
                            app.message = format!("ERROR: {}", e);
                        }
                    }
                }
                app.screen = Screen::Success;
                false
            }
            KeyCode::Esc => {
                app.screen = Screen::ProjectName;
                false
            }
            _ => false,
        },
        Screen::Success => match key.code {
            KeyCode::Char('q') | KeyCode::Enter | KeyCode::Esc => true,
            _ => false,
        },
    }
}

fn render_frame(f: &mut Frame, app: &App) {
    match app.screen {
        Screen::Welcome => render_welcome(f, app),
        Screen::ProjectName => render_project_name(f, app),
        Screen::TemplateSelect => render_template_select(f, app),
        Screen::Success => render_success(f, app),
    }
}

fn render_welcome(f: &mut Frame, _app: &App) {
    let size = f.size();
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Min(10),
            Constraint::Fill(1),
        ])
        .split(size);

    let title_lines = vec![
        Line::from(Span::styled(
            "┌──────────────────────────────────────────────────┐",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "│              mg24-generate v0.5.0                 │",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "│         Interactive Project Generator             │",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "│            for mg24-hal EFR32MG24                 │",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "└──────────────────────────────────────────────────┘",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  Create new mg24-hal projects with pre-configured templates"),
        Line::from(""),
        Line::from(Span::styled(
            "  Press <ENTER> to continue",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::DIM),
        )),
    ];

    let title = Paragraph::new(title_lines).alignment(Alignment::Center);
    f.render_widget(title, vertical[1]);
}

fn render_project_name(f: &mut Frame, app: &App) {
    let size = f.size();
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Min(8),
            Constraint::Length(2),
            Constraint::Fill(1),
        ])
        .split(size);

    let input_lines = vec![
        Line::from(""),
        Line::from("  Enter project name:"),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}{}", app.project_name, "▌"),
            Style::default().fg(Color::White).bg(Color::Black),
        )),
        Line::from(""),
    ];

    let input_block = Block::default()
        .title(" Project Name ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Thick)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    let input = Paragraph::new(input_lines).block(input_block);
    f.render_widget(input, vertical[1]);

    let help = Paragraph::new(vec![Line::from(Span::styled(
        "Type project name and press <ENTER>  |  <ESC> to go back",
        Style::default().fg(Color::DarkGray),
    ))]).alignment(Alignment::Center);
    f.render_widget(help, vertical[2]);
}

fn render_template_select(f: &mut Frame, app: &App) {
    let size = f.size();
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Min(10),
            Constraint::Length(2),
            Constraint::Fill(1),
        ])
        .split(size);

    let descriptions = [
        "Empty project with initialization",
        "LED blinking example",
        "Button input with LED control",
        "I2C communication example",
        "DMA memory transfer example",
    ];

    let items: Vec<ListItem> = app
        .templates
        .iter()
        .enumerate()
        .map(|(idx, template)| {
            let desc = descriptions[idx];
            let content = format!("  {:12} {}", template, desc);

            if idx == app.selected_template {
                ListItem::new(Span::styled(
                    content,
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
            } else {
                ListItem::new(content)
            }
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" Select Template ")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Thick)
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    );

    f.render_widget(list, vertical[1]);

    let help = Paragraph::new(vec![Line::from(Span::styled(
        "Use ↑↓ or j/k to navigate, <ENTER> to select, <ESC> to go back",
        Style::default().fg(Color::DarkGray),
    ))]).alignment(Alignment::Center);
    f.render_widget(help, vertical[2]);
}

fn render_success(f: &mut Frame, app: &App) {
    let size = f.size();
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Min(10),
            Constraint::Length(2),
            Constraint::Fill(1),
        ])
        .split(size);

    let msg_lines: Vec<Line> = app
        .message
        .split('\n')
        .map(|line| Line::from(line.to_string()))
        .collect();

    let msg_block = Block::default()
        .title(" Result ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Thick)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    let msg = Paragraph::new(msg_lines).block(msg_block).alignment(Alignment::Center);
    f.render_widget(msg, vertical[1]);

    let help = Paragraph::new(vec![Line::from(Span::styled(
        "Press <ENTER> or <Q> to exit",
        Style::default().fg(Color::DarkGray),
    ))]).alignment(Alignment::Center);
    f.render_widget(help, vertical[2]);
}
