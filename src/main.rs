mod app;
mod config;
mod ui;

use app::{App, Mode};
use config::Config;
use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io, process::Command};
use ui::ui;

const CONFIG_SEARCH_PATHS: &[&str] = &[
    ".local/bolt.yml",
    ".local/bolt.yaml",
    "bolt.yml",
    "bolt.yaml",
];

// Restores the terminal to its original state on drop, ensuring cleanup even on
// early returns or panics between enable_raw_mode() and the matching teardown.
struct TerminalCleanup;

impl Drop for TerminalCleanup {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            cursor::Show
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let explicit_config = args.windows(2).find_map(|w| {
        if w[0] == "--config" { Some(w[1].clone()) } else { None }
    });

    let candidates: Vec<String> = if let Some(path) = explicit_config {
        vec![path]
    } else {
        CONFIG_SEARCH_PATHS.iter().map(|s| s.to_string()).collect()
    };

    let mut loaded_config: Option<Config> = None;
    let mut used_path = String::new();

    for path in &candidates {
        if std::path::Path::new(path).exists() {
            match Config::load(path) {
                Ok(cfg) => {
                    loaded_config = Some(cfg);
                    used_path = path.clone();
                    break;
                }
                Err(e) => {
                    eprintln!("Error parsing {}: {}", path, e);
                }
            }
        }
    }

    let config = match loaded_config {
        Some(cfg) => cfg,
        None => {
            eprintln!("Could not find bolt.yml in .local/ or project root. Use --config <path> to specify a custom location.");
            return Ok(());
        }
    };

    if config.tasks.is_empty() {
        eprintln!("Warning: no tasks found in {}", used_path);
        return Ok(());
    }

    println!("Loaded tasks from {}", used_path);

    // When a child process runs (raw mode disabled), Ctrl+C sends SIGINT to the
    // whole process group. The child handles it; the parent should ignore it and
    // return to the menu.
    if let Err(e) = ctrlc::set_handler(move || {}) {
        eprintln!("Warning: could not set Ctrl+C handler: {}", e);
    }

    let mut app = App::new(config.tasks);

    loop {
        app.reset();

        enable_raw_mode()?;
        // Guard ensures disable_raw_mode + LeaveAlternateScreen run on any exit path.
        let cleanup = TerminalCleanup;

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = run_app(&mut terminal, &mut app);
        drop(cleanup); // Restore terminal before writing to stdout below.

        if let Err(err) = res {
            eprintln!("{:?}", err);
            break;
        }

        if let Some(cmd_str) = &app.selected_command {
            println!("> Running: {}", cmd_str);

            let status = if cfg!(windows) {
                Command::new("cmd").args(["/C", cmd_str]).status()
            } else {
                Command::new("sh").arg("-c").arg(cmd_str).status()
            };

            match status {
                Ok(s) => {
                    if !s.success() {
                        eprintln!("Command exited with status: {}", s);
                    }
                }
                Err(e) => eprintln!("Failed to execute command: {}", e),
            }

            println!("\nPress Enter to return to menu, or 'q' / Esc to quit...");

            enable_raw_mode()?;
            let should_quit = loop {
                match event::read() {
                    Ok(Event::Key(key)) => {
                        break matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
                            || (key.code == KeyCode::Char('c')
                                && key.modifiers.contains(event::KeyModifiers::CONTROL));
                    }
                    Ok(_) => continue,
                    Err(e) => {
                        let _ = disable_raw_mode(); // best-effort cleanup before propagating
                        return Err(e.into());
                    }
                }
            };
            disable_raw_mode()?;

            if should_quit {
                return Ok(());
            }
        } else {
            break;
        }
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()>
where
    io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                Mode::Browse => match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Enter => app.execute_selected(),
                    KeyCode::Up => app.select_previous(),
                    KeyCode::Down => app.select_next(),
                    KeyCode::Backspace => app.on_backspace(),
                    KeyCode::Char(c) => app.on_key(c),
                    _ => {}
                },
                Mode::ParamInput => match key.code {
                    KeyCode::Esc => app.cancel_param_input(),
                    KeyCode::Enter => app.on_param_enter(),
                    KeyCode::Backspace => app.on_param_backspace(),
                    KeyCode::Char(c) => app.on_param_key(c),
                    _ => {}
                },
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
