use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, time::Duration};
use sysinfo::{CpuExt, System, SystemExt};

struct App {
    system: System,
}

impl App {
    fn new() -> Self {
        let mut system = System::new();
        system.refresh_all();
        Self { system }
    }

    fn update(&mut self) {
        self.system.refresh_all();
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn create_cpu_info(app: &App) -> Vec<Spans> {
    let mut text = Vec::new();
    
    // Overall CPU Usage
    let cpu_usage = app.system.global_cpu_info().cpu_usage();
    text.push(Spans::from(vec![
        Span::raw("Overall CPU Usage: "),
        Span::styled(
            format!("{:.1}%", cpu_usage),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        ),
    ]));
    text.push(Spans::from(""));

    // Per-core usage
    text.push(Spans::from(Span::styled(
        "Per Core Usage:",
        Style::default().add_modifier(Modifier::UNDERLINED)
    )));
    
    for (i, cpu) in app.system.cpus().iter().enumerate() {
        let usage = cpu.cpu_usage();
        text.push(Spans::from(vec![
            Span::raw(format!("Core {}: ", i)),
            Span::styled(
                format!("{:.1}%", usage),
                Style::default().fg(Color::Green)
            ),
        ]));
    }

    text
}

fn create_memory_info(app: &App) -> Vec<Spans> {
    let mut text = Vec::new();
    
    let total_mem = app.system.total_memory() as f64 / 1024.0 / 1024.0; // MB
    let used_mem = app.system.used_memory() as f64 / 1024.0 / 1024.0;
    let free_mem = app.system.free_memory() as f64 / 1024.0 / 1024.0;
    let available_mem = app.system.available_memory() as f64 / 1024.0 / 1024.0;

    text.push(Spans::from(vec![
        Span::raw("Total Memory: "),
        Span::styled(
            format!("{:.1} MB", total_mem),
            Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)
        ),
    ]));

    text.push(Spans::from(vec![
        Span::raw("Used Memory: "),
        Span::styled(
            format!("{:.1} MB", used_mem),
            Style::default().fg(Color::Red)
        ),
    ]));

    text.push(Spans::from(vec![
        Span::raw("Free Memory: "),
        Span::styled(
            format!("{:.1} MB", free_mem),
            Style::default().fg(Color::Green)
        ),
    ]));

    text.push(Spans::from(vec![
        Span::raw("Available Memory: "),
        Span::styled(
            format!("{:.1} MB", available_mem),
            Style::default().fg(Color::Blue)
        ),
    ]));

    text
}

fn run_app<B: tui::backend::Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(f.size());

            // CPU Info
            let cpu_info = Paragraph::new(create_cpu_info(&app))
                .block(Block::default().title("CPU Information").borders(Borders::ALL))
                .style(Style::default().fg(Color::White));
            f.render_widget(cpu_info, chunks[0]);

            // Memory Info
            let memory_info = Paragraph::new(create_memory_info(&app))
                .block(Block::default().title("Memory Information").borders(Borders::ALL))
                .style(Style::default().fg(Color::White));
            f.render_widget(memory_info, chunks[1]);
        })?;

        app.update();

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
    }
}
