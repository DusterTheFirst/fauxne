use logger::{logger_text, TuiLogger};
use std::{
    io::{self, stdout, Stdout},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    thread::{self, JoinHandle},
    time::Duration,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, List, Paragraph, Text, Widget},
    Terminal,
};

pub mod logger;

pub struct UI {
    terminal: RwLock<Terminal<CrosstermBackend<Stdout>>>,
    tick_rate: u64,
    running: AtomicBool,
}

impl UI {
    pub fn new(tick_rate: u64) -> Arc<Self> {
        // Construct the terminal
        let stdout = stdout();
        let backend = CrosstermBackend::new(stdout);
        Arc::new(Self {
            terminal: RwLock::new(Terminal::new(backend).expect("failed to bind to terminal")),
            running: AtomicBool::new(true),
            tick_rate,
        })
    }

    pub fn run(self: Arc<Self>) -> JoinHandle<io::Result<()>> {
        thread::spawn(move || {
            {
                let mut term_lock = self.terminal.write().unwrap();
                term_lock.clear()?;
                term_lock.hide_cursor()?;
            }

            while self.running.load(Ordering::Relaxed) {
                self.render()?;

                {
                    let mut term_lock = self.terminal.write().unwrap();
                    let (_, cy) = term_lock.get_cursor()?;
                    term_lock.set_cursor(0, cy + 1)?;
                }

                thread::sleep(Duration::from_millis(self.tick_rate));
            }

            Ok(())
        })
    }

    pub fn stop(self: Arc<Self>) -> io::Result<()> {
        self.running.store(false, Ordering::Relaxed);

        let mut term_lock = self.terminal.write().unwrap();
        term_lock.show_cursor()?;

        Ok(())
    }

    pub fn render(&self) -> io::Result<()> {
        self.terminal.write().unwrap().draw(|mut f| {
            f.render_widget(Clear, f.size());

            let chunks_h = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());
            let chunks_v = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks_h[1]);

            let text = logger_text(chunks_h[0].height);
            let console_output = Paragraph::new(text.iter())
                .block(Block::default().title("Paragraph").borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Left)
                .wrap(true).raw(true);
            f.render_widget(console_output, chunks_h[0]);

            let state_viewer = Block::default().title("State").borders(Borders::ALL);
            f.render_widget(state_viewer, chunks_v[0]);

            let console_exe = Block::default()
                .title("Command Console")
                .borders(Borders::ALL);
            f.render_widget(console_exe, chunks_v[1]);
        })?;

        Ok(())
    }
}
