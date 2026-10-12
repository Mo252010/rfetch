mod collector;
mod config;
mod gpu;
mod logo;
mod theme;
mod ui;

use anyhow::Result;
use collector::{Collector, Snapshot};
use config::Config;
use crossterm::cursor::{MoveToColumn, Show};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::{DefaultTerminal, Terminal, TerminalOptions, Viewport};
use std::io::Stdout;
use ui::display::Action;

const HELP: &str = "\
rfetch - a fast, live system info TUI with a built-in config editor

USAGE:
    rfetch [OPTIONS]

OPTIONS:
    -g, --gui        Open the config editor first
    -o, --once       Print one snapshot as plain text and exit
    -p, --path       Print the config file path and exit
    -h, --help       Print this help and exit
    -V, --version    Print version and exit

KEYBINDINGS (live view):
    e                open config editor
    q / Esc          quit
    Ctrl-C           quit

ENVIRONMENT:
    RFETCH_ASCII     Override the detected logo (an OS or distro name, e.g.
                     omarchy | arch | ubuntu | macos | windows)

KEYBINDINGS (editor):
    1/2/3 or Tab     switch section (General / Theme / Modules)
    Up/Down          select field or module
    Left/Right       change value / cycle palette / bar width
    Space/Enter      toggle or activate
    a / d            add / delete module
    J / K            move module down / up
    n / m            edit module label / mounts
    s                save config
    q / Esc          quit editor
";

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{HELP}");
        return Ok(());
    }
    if args.iter().any(|a| a == "-V" || a == "--version") {
        println!("rfetch {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    if args.iter().any(|a| a == "-p" || a == "--path") {
        println!("{}", config::config_path().display());
        return Ok(());
    }

    let editor_first = args.iter().any(|a| a == "-g" || a == "--gui");

    let mut config = config::load()?;
    let mut collector = Collector::new();

    if args.iter().any(|a| a == "-o" || a == "--once") {
        collector.refresh();
        std::thread::sleep(std::time::Duration::from_millis(300));
        let snapshot = collector.refresh();
        print!("{}", ui::display::plain_text(&config, &snapshot));
        return Ok(());
    }

    collector.refresh();
    std::thread::sleep(std::time::Duration::from_millis(200));
    let snapshot = collector.refresh();

    let mut height = display_height(&config, &snapshot);
    let mut terminal = init_inline(height);

    if editor_first {
        open_editor(&mut config)?;
        let snapshot = collector.refresh();
        let new_height = display_height(&config, &snapshot);
        if new_height != height {
            height = new_height;
            resize_inline(&mut terminal, height);
        }
    }

    let result = loop {
        match ui::display::run(&mut terminal, &config, &mut collector) {
            Ok(Action::Quit) => break Ok(()),
            Ok(Action::OpenEditor) => {
                if let Err(e) = open_editor(&mut config) {
                    break Err(e);
                }
                let snapshot = collector.refresh();
                let new_height = display_height(&config, &snapshot);
                if new_height != height {
                    height = new_height;
                    resize_inline(&mut terminal, height);
                }
            }
            Err(e) => break Err(e),
        }
    };

    ratatui::restore();
    let _ = crossterm::execute!(std::io::stdout(), Show, MoveToColumn(0), crossterm::style::Print("\n"));

    result
}

/// Content height, clamped so it always fits within the terminal.
fn display_height(config: &Config, snapshot: &Snapshot) -> u16 {
    let (width, term_height) = crossterm::terminal::size().unwrap_or((100, 40));
    let wanted = ui::display::content_height(config, snapshot, width);
    wanted.clamp(1, term_height.saturating_sub(1).max(1))
}

fn init_inline(height: u16) -> DefaultTerminal {
    ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(height),
    })
}

/// The inline viewport height is fixed for a terminal's lifetime, so when the
/// layout changes we finish the current viewport and start a fresh one.
fn resize_inline(terminal: &mut DefaultTerminal, height: u16) {
    ratatui::restore();
    let _ = crossterm::execute!(std::io::stdout(), Show, MoveToColumn(0), crossterm::style::Print("\n"));
    *terminal = init_inline(height);
}

/// Opens the config editor full-screen (alternate screen), then leaves it,
/// returning to the inline live view.
fn open_editor(config: &mut Config) -> Result<()> {
    let mut out = std::io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(out, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal: Terminal<CrosstermBackend<Stdout>> = Terminal::new(backend)?;
    let result = ui::editor::run(&mut terminal, config);
    crossterm::execute!(out, LeaveAlternateScreen)?;
    *config = config::load()?;
    result
}
