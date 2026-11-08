pub mod app;
pub mod db;
pub mod logger;
pub mod models;
pub mod ui;
pub mod web;

use app::{ActivePane, App, InputMode};
use crossterm::{
    cursor::Show,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use db::Database;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::panic;
use std::time::Duration;

/// Runs the complete DhabaTUI POS application lifecycle.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize file-based logger (dhabatui.log)
    let _log_guard = logger::init_logger(".")?;
    tracing::info!("Starting DhabaTUI POS System");

    // 2. Setup custom panic hook to restore terminal on unexpected crashes
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            Show
        );
        tracing::error!(panic = ?panic_info, "Fatal panic in DhabaTUI");
        original_hook(panic_info);
    }));

    // 3. Open redb database
    let db = Database::open("dhabatui.redb")?;
    let mut app = App::new(db)?;

    // 4. Initialize terminal in raw mode and alternate screen
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 5. Main event loop
    let res = run_app(&mut terminal, &mut app);

    // 6. Restore canonical terminal state
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        Show
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Application Error: {err}");
        tracing::error!(%err, "Application exited with error");
    } else {
        tracing::info!("DhabaTUI exited cleanly");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    while !app.should_quit {
        terminal.draw(|f| ui::render(f, app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                // Ignore key release events (only handle Press / Repeat)
                if key.kind == KeyEventKind::Release {
                    continue;
                }

                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.should_quit = true;
                        }
                        KeyCode::Char('t') | KeyCode::Char('T') => {
                            app.next_table();
                        }
                        KeyCode::BackTab => {
                            app.previous_table();
                        }
                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            app.toggle_active_table_status();
                        }
                        KeyCode::Tab => {
                            app.toggle_pane();
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                            app.active_pane = ActivePane::Menu;
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            app.active_pane = ActivePane::Cart;
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.next_item();
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.previous_item();
                        }
                        KeyCode::Enter => {
                            if app.active_pane == ActivePane::Menu {
                                app.add_selected_menu_item();
                            }
                        }
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            app.increment_selected_cart_item();
                        }
                        KeyCode::Char('-') => {
                            app.decrement_selected_cart_item();
                        }
                        KeyCode::Char('d') | KeyCode::Char('D') => {
                            app.delete_selected_cart_item();
                        }
                        KeyCode::Char('c') | KeyCode::Char('C') => {
                            app.clear_cart();
                        }
                        KeyCode::Char('e') | KeyCode::Char('E') => {
                            app.start_editing_selected_price();
                        }
                        KeyCode::Char('m') | KeyCode::Char('M') => {
                            app.start_custom_item();
                        }
                        KeyCode::Char('p') | KeyCode::Char('P') => {
                            let _ = app.settle_and_print();
                        }
                        KeyCode::Char(c) if c.is_ascii_digit() => {
                            app.input_mode = InputMode::QuickCode;
                            app.quick_code_buffer.clear();
                            app.quick_code_buffer.push(c);
                        }
                        _ => {}
                    },

                    InputMode::QuickCode => match key.code {
                        KeyCode::Char(c) if c.is_ascii_digit() => {
                            app.quick_code_buffer.push(c);
                        }
                        KeyCode::Backspace => {
                            app.quick_code_buffer.pop();
                            if app.quick_code_buffer.is_empty() {
                                app.input_mode = InputMode::Normal;
                            }
                        }
                        KeyCode::Esc => {
                            app.quick_code_buffer.clear();
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Enter => {
                            let code = std::mem::take(&mut app.quick_code_buffer);
                            app.add_item_by_code(&code);
                            app.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },

                    InputMode::EditingPrice => match key.code {
                        KeyCode::Char(c) if c.is_ascii_digit() || c == '.' || c == '₹' => {
                            app.input_buffer.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input_buffer.pop();
                        }
                        KeyCode::Esc => {
                            app.input_buffer.clear();
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Enter => {
                            app.commit_edited_price();
                        }
                        _ => {}
                    },

                    InputMode::AddingCustomName => match key.code {
                        KeyCode::Char(c) => {
                            app.input_buffer.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input_buffer.pop();
                        }
                        KeyCode::Esc => {
                            app.input_buffer.clear();
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Enter => {
                            app.commit_custom_name();
                        }
                        _ => {}
                    },

                    InputMode::AddingCustomPrice => match key.code {
                        KeyCode::Char(c) if c.is_ascii_digit() || c == '.' || c == '₹' => {
                            app.input_buffer.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input_buffer.pop();
                        }
                        KeyCode::Esc => {
                            app.input_buffer.clear();
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Enter => {
                            app.commit_custom_price();
                        }
                        _ => {}
                    },
                }
            }
        }

        // Periodic maintenance (e.g. toast auto-expiration)
        app.tick();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_documented_shortcuts_consistency() {
        const README: &str = include_str!("../README.md");
        const SITE: &str = include_str!("../site/index.html");
        const WIKI_OPS: &str = include_str!("../wiki/Cashier-and-Staff-Operations.md");

        // Verify version alignment in documentation website
        let pkg_version = format!("v{}", env!("CARGO_PKG_VERSION"));
        assert!(
            SITE.contains(&pkg_version),
            "site/index.html must display Cargo.toml package version ({pkg_version})"
        );

        // Verify README documentation accuracy
        assert!(
            README.contains("<kbd>t</kbd>"),
            "README must document 't' for table cycling"
        );
        assert!(
            README.contains("<kbd>Tab</kbd>"),
            "README must document 'Tab' for pane focus"
        );
        assert!(
            README.contains("<kbd>s</kbd>"),
            "README must document 's' for table status"
        );
        assert!(
            !README.contains("<kbd>Tab</kbd> | Cycle Tables")
                && !README.contains("single press of the `[Tab]` key"),
            "README must NOT document Tab for cycling tables (regression prevention)"
        );
        assert!(
            !README.contains("print an itemized bill") && !README.contains("Print Bill & Settle"),
            "README must NOT claim hardware printing (app exports receipt.txt)"
        );
        assert!(
            !README.contains("2-digit dish numbers"),
            "README must document 3-digit dish codes (e.g. 101)"
        );

        // Verify documentation website accuracy
        assert!(
            SITE.contains("<kbd>t</kbd>"),
            "site/index.html must document 't' for table cycling"
        );
        assert!(
            SITE.contains("<kbd>Tab</kbd>"),
            "site/index.html must document 'Tab' for pane focus"
        );
        assert!(
            SITE.contains("<kbd>s</kbd>"),
            "site/index.html must document 's' for table status"
        );
        assert!(
            !SITE.contains("zero rounding errors"),
            "site/index.html must NOT claim zero rounding errors (GST uses integer half-up rounding)"
        );
        assert!(
            !SITE.contains("kitchen tickets"),
            "site/index.html must NOT claim kitchen tickets (app has read-only table dashboard)"
        );
        assert!(
            !SITE.contains("Print Bill & Settle"),
            "site/index.html must NOT claim hardware printing"
        );

        // Verify Wiki operations guide accuracy
        assert!(
            WIKI_OPS.contains("`[t]` / `[T]`"),
            "wiki must document 't' / 'T' for table cycling"
        );
        assert!(
            WIKI_OPS.contains("`[Tab]`"),
            "wiki must document 'Tab' for pane focus"
        );
        assert!(
            WIKI_OPS.contains("`[s]`"),
            "wiki must document 's' for table status"
        );
    }
}
