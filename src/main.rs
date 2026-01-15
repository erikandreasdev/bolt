mod app;
mod config;
mod ui;

use anyhow::Result;
use app::App;
use config::Config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io, process::Command};
use ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Load configuration
    let config_files = ["bolt.yml", "bolt.yaml"];
    let mut loaded_config = None;
    let mut used_path = "";

    for path in config_files {
        if std::path::Path::new(path).exists() {
            match Config::load(path) {
                Ok(cfg) => {
                    loaded_config = Some(cfg);
                    used_path = path;
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
            eprintln!("Could not find or parse bolt.yml or bolt.yaml");
            return Ok(());
        }
    };
    
    println!("Loaded tasks from {}", used_path);

    // Setup signal handler to ignore Ctrl+C (SIGINT) in the parent process.
    // When a child process is running (raw mode disabled), Ctrl+C sends SIGINT to the process group.
    // The child will terminate, and this handler ensures the parent (devrunner) ignores it and continues.
    // In raw mode (TUI), Ctrl+C is just a key event and no SIGINT is generated.
    let _ = ctrlc::set_handler(move || { });

    // 3. Create App state
    let mut app = App::new(config.tasks);

    loop {
        // Reset state for new run loop
        app.should_quit = false;
        app.selected_command = None;

        // 4. Setup Terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // 5. Run Main Loop
        let res = run_app(&mut terminal, &mut app);

        // 6. Restore Terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            eprintln!("{:?}", err);
            break;
        }

        // 7. Execute command if one was selected
        if let Some(cmd_str) = &app.selected_command {
            println!("> Running: {}", cmd_str);
            
            // Allow signals (like Ctrl+C) to be handled by the child process
            // We don't need special handling here because we are not in raw mode anymore.

            let status = Command::new("sh")
                .arg("-c")
                .arg(cmd_str)
                .status();

            match status {
                 Ok(s) => {
                     if !s.success() {
                         eprintln!("Command exited with status: {}", s);
                     }
                 }
                 Err(e) => eprintln!("Failed to execute command: {}", e),
            }
            
            println!("\nPress Enter to return to menu, or 'q' / Esc to quit...");
            
            // Re-enable raw mode to capture single keypress
            enable_raw_mode()?;
            loop {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            disable_raw_mode()?;
                            return Ok(());
                        }
                        KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                             disable_raw_mode()?;
                             return Ok(());
                        }
                        _ => {
                            disable_raw_mode()?;
                            break; 
                        }
                    }
                }
            }
        } else {
            // User quit without selecting a command (Esc)
            break;
        }
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Enter => {
                    app.execute_selected();
                }
                KeyCode::Up => app.select_previous(),
                KeyCode::Down => app.select_next(),
                KeyCode::Backspace => app.on_backspace(),
                KeyCode::Char(c) => app.on_key(c),
                _ => {}
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
