use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, List, ListItem},
    Terminal, Frame,
};
use std::io;
use std::path::PathBuf;

use crate::generator;

enum Screen {
    Welcome,
    ProjectName,
    TemplateSelect,
    Success(String),
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

struct App {
    screen: Screen,
    project_name: String,
    templates: Vec<&'static str>,
    selected_template: usize,
}

impl App {
    fn new() -> Self {
        Self {
            screen: Screen::Welcome,
            project_name: String::new(),
            templates: vec!["blank", "blink", "button", "i2c", "dma"],
            selected_template: 0,
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match &app.screen {
                    Screen::Welcome => {
                        if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                            return Ok(());
                        }
                        if key.code == KeyCode::Enter {
                            app.screen = Screen::ProjectName;
                        }
                    }
                    Screen::ProjectName => match key.code {
                        KeyCode::Char(c) => app.project_name.push(c),
                        KeyCode::Backspace => {
                            app.project_name.pop();
                        }
                        KeyCode::Enter => {
                            if !app.project_name.is_empty() {
                                app.screen = Screen::TemplateSelect;
                            }
                        }
                        KeyCode::Esc => {
                            app.screen = Screen::Welcome;
                            app.project_name.clear();
                        }
                        _ => {}
                    },
                    Screen::TemplateSelect => match key.code {
                        KeyCode::Up => {
                            if app.selected_template > 0 {
                                app.selected_template -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if app.selected_template < app.templates.len() - 1 {
                                app.selected_template += 1;
                            }
                        }
                        KeyCode::Enter => {
                            let template = app.templates[app.selected_template];
                            let project_path = PathBuf::from(&app.project_name);

                            if project_path.exists() {
                                let msg = format!("ERROR: Directory '{}' already exists!", app.project_name);
                                app.screen = Screen::Success(msg);
                            } else {
                                match generator::create_project(&project_path, template) {
                                    Ok(()) => {
                                        let msg = format!(
                                            "SUCCESS: Project '{}' created with '{}' template!\n\ncd {}  &&  cargo build --release",
                                            app.project_name, template, app.project_name
                                        );
                                        app.screen = Screen::Success(msg);
                                    }
                                    Err(e) => {
                                        let msg = format!("ERROR: {}", e);
                                        app.screen = Screen::Success(msg);
                                    }
                                }
                            }
                        }
                        KeyCode::Esc => {
                            app.screen = Screen::ProjectName;
                        }
                        _ => {}
                    },
                    Screen::Success(_) => {
                        if key.code == KeyCode::Char('q') || key.code == KeyCode::Enter || key.code == KeyCode::Esc {
                            return Ok(());
                        }
                    }
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.size();

    match &app.screen {
        Screen::Welcome => draw_welcome(f, size),
        Screen::ProjectName => draw_project_name(f, size, app),
        Screen::TemplateSelect => draw_template_select(f, size, app),
        Screen::Success(msg) => draw_success(f, size, msg),
    }
}

fn draw_welcome(f: &mut Frame, size: ratatui::layout::Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(size);

    let title = vec![
        Line::from(""),
    ];
    let title_para = Paragraph::new(title)
        .alignment(Alignment::Center);
    f.render_widget(title_para, chunks[0]);

    let welcome_text = vec![
        Line::from(Span::styled(
            "╔══════════════════════════════════════════════════╗",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            "║                   mg24-generate                  ║",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            "║          Interactive Project Generator           ║",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            "║              for mg24-hal EFR32MG24              ║",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            "╚══════════════════════════════════════════════════╝",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from("Create new mg24-hal projects with templates."),
        Line::from(""),
    ];

    let welcome_para = Paragraph::new(welcome_text)
        .block(Block::default())
        .alignment(Alignment::Center);
    f.render_widget(welcome_para, chunks[1]);

    let help_text = vec![
        Line::from(Span::styled(
            "Press <ENTER> to continue or <Q> to quit",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::DIM),
        )),
    ];
    let help_para = Paragraph::new(help_text)
        .alignment(Alignment::Center);
    f.render_widget(help_para, chunks[2]);
}

fn draw_project_name(f: &mut Frame, size: ratatui::layout::Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(7),
            Constraint::Length(2),
        ])
        .split(size);

    let input_content = vec![
        Line::from(""),
        Line::from("Enter project name:"),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}{}", app.project_name, "█"),
            Style::default().fg(Color::White).bg(Color::Black),
        )),
        Line::from(""),
    ];

    let input_block = Block::default()
        .title(" Project Name ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Plain)
        .style(Style::default().fg(Color::Cyan));

    let input_para = Paragraph::new(input_content)
        .block(input_block)
        .alignment(Alignment::Left);
    f.render_widget(input_para, chunks[1]);

    let help_text = vec![
        Line::from(Span::styled(
            "Type project name, press <ENTER> to continue, <ESC> to back",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::DIM),
        )),
    ];
    let help_para = Paragraph::new(help_text)
        .alignment(Alignment::Center);
    f.render_widget(help_para, chunks[2]);
}

fn draw_template_select(f: &mut Frame, size: ratatui::layout::Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(12),
            Constraint::Length(2),
        ])
        .split(size);

    let templates_desc = [
        ("blank", "Empty project template"),
        ("blink", "LED blinking example"),
        ("button", "Button input with LED control"),
        ("i2c", "I2C communication example"),
        ("dma", "DMA memory transfer example"),
    ];

    let items: Vec<ListItem> = app
        .templates
        .iter()
        .enumerate()
        .map(|(idx, template)| {
            let desc = templates_desc[idx].1;
            let content = format!("  {}  -  {}", template, desc);

            if idx == app.selected_template {
                ListItem::new(Span::styled(
                    content,
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
            } else {
                ListItem::new(Span::raw(content))
            }
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Select Template ")
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Plain)
                .style(Style::default().fg(Color::Cyan)),
        );

    f.render_widget(list, chunks[1]);

    let help_text = vec![
        Line::from(Span::styled(
            "Use UP/DOWN arrows to select, <ENTER> to create, <ESC> to back",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::DIM),
        )),
    ];
    let help_para = Paragraph::new(help_text)
        .alignment(Alignment::Center);
    f.render_widget(help_para, chunks[2]);
}

fn draw_success(f: &mut Frame, size: ratatui::layout::Rect, message: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(10),
            Constraint::Length(2),
        ])
        .split(size);

    let lines: Vec<Line> = message
        .split('\n')
        .map(|line| Line::from(line.to_string()))
        .collect();

    let msg_block = Block::default()
        .title(" Result ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Plain)
        .style(Style::default().fg(Color::Cyan));

    let msg_para = Paragraph::new(lines)
        .block(msg_block)
        .alignment(Alignment::Center);
    f.render_widget(msg_para, chunks[1]);

    let help_text = vec![
        Line::from(Span::styled(
            "Press <ENTER> or <Q> to exit",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::DIM),
        )),
    ];
    let help_para = Paragraph::new(help_text)
        .alignment(Alignment::Center);
    f.render_widget(help_para, chunks[2]);
}
