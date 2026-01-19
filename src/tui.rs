use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct FileBrowser {
    current_path: PathBuf,
    items: Vec<FileItem>,
    state: ListState,
    show_hidden: bool,
}

#[derive(Clone)]
struct FileItem {
    name: String,
    path: PathBuf,
    is_dir: bool,
    size: Option<u64>,
}

impl FileBrowser {
    pub fn new(path: PathBuf) -> Result<Self> {
        let mut browser = Self {
            current_path: path,
            items: Vec::new(),
            state: ListState::default(),
            show_hidden: false,
        };
        browser.refresh()?;
        browser.state.select(Some(0));
        Ok(browser)
    }

    fn refresh(&mut self) -> Result<()> {
        self.items.clear();

        // Add parent directory entry if not at root
        if self.current_path.parent().is_some() {
            self.items.push(FileItem {
                name: "..".to_string(),
                path: self.current_path.parent().unwrap().to_path_buf(),
                is_dir: true,
                size: None,
            });
        }

        // Read directory entries
        let entries = fs::read_dir(&self.current_path)?;

        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();

            // Skip hidden files if not showing them
            if !self.show_hidden && name.starts_with('.') {
                continue;
            }

            let metadata = entry.metadata().ok();
            let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
            let size = metadata.as_ref().map(|m| m.len());

            self.items.push(FileItem {
                name,
                path: entry.path(),
                is_dir,
                size,
            });
        }

        // Sort: directories first, then alphabetically
        self.items.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });

        Ok(())
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn enter(&mut self) -> Result<()> {
        if let Some(i) = self.state.selected()
            && let Some(item) = self.items.get(i)
            && item.is_dir
        {
            self.current_path = item.path.clone();
            self.refresh()?;
            self.state.select(Some(0));
        }
        Ok(())
    }

    fn toggle_hidden(&mut self) -> Result<()> {
        self.show_hidden = !self.show_hidden;
        self.refresh()?;
        self.state.select(Some(0));
        Ok(())
    }
}

pub fn run_tui(start_path: &str) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create file browser
    let path = PathBuf::from(start_path);
    let path = if path.exists() {
        if path.is_dir() {
            path
        } else {
            path.parent().unwrap_or(Path::new(".")).to_path_buf()
        }
    } else {
        PathBuf::from(".")
    };

    let mut browser = FileBrowser::new(path)?;

    // Main loop
    loop {
        terminal.draw(|f| ui(f, &mut browser))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                KeyCode::Down | KeyCode::Char('j') => browser.next(),
                KeyCode::Up | KeyCode::Char('k') => browser.previous(),
                KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => browser.enter()?,
                KeyCode::Char('h') => browser.toggle_hidden()?,
                KeyCode::Char('r') => browser.refresh()?,
                _ => {}
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, browser: &mut FileBrowser) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                "Ferret ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("File Browser", Style::default().fg(Color::White)),
        ]),
        Line::from(Span::styled(
            format!("📁 {}", browser.current_path.display()),
            Style::default().fg(Color::Yellow),
        )),
    ])
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // File list
    let items: Vec<ListItem> = browser
        .items
        .iter()
        .map(|item| {
            let icon = if item.is_dir { "📁" } else { "📄" };
            let size_str = if let Some(size) = item.size {
                format!(" ({})", humansize::format_size(size, humansize::BINARY))
            } else {
                String::new()
            };

            let style = if item.is_dir {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(format!("{} {}{}", icon, item.name, size_str)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Files"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, chunks[1], &mut browser.state);

    // Footer with help
    let help_text = if browser.show_hidden {
        "↑↓/jk: Navigate | Enter/l: Open | h: Hide hidden | r: Refresh | q/Esc: Quit"
    } else {
        "↑↓/jk: Navigate | Enter/l: Open | h: Show hidden | r: Refresh | q/Esc: Quit"
    };

    let footer = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
