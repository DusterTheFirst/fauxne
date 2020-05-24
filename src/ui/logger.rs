use crate::circular::CircularBuffer;
use chrono::{DateTime, Local};
use lazy_static::lazy_static;
use log::{set_logger, set_max_level, Level, LevelFilter, Log, Metadata, Record};
use std::{
    cmp::min,
    sync::{RwLock, RwLockWriteGuard},
};
use tui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Alignment, Rect},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Text, Widget},
    Terminal,
};

lazy_static! {
    static ref TUI_LOGGER: TuiLogger = TuiLogger {
        conf: RwLock::new(TuiLoggerConfig {
            level: LevelFilter::Trace,
            target_filter: TargetFilter::None
        }),
        messages: RwLock::new(CircularBuffer::new(128))
    };
}

#[derive(Debug)]
struct LogMessage {
    pub timestamp: DateTime<Local>,
    pub level: Level,
    pub target: String,
    pub file: String,
    pub line: u32,
    pub message: String,
}

#[derive(Debug)]
pub struct TuiLogger {
    pub conf: RwLock<TuiLoggerConfig>,
    messages: RwLock<CircularBuffer<LogMessage>>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct TuiLoggerConfig {
    pub level: LevelFilter,
    pub target_filter: TargetFilter,
}

impl Default for TuiLoggerConfig {
    fn default() -> Self {
        Self {
            level: LevelFilter::Trace,
            target_filter: Default::default(),
        }
    }
}

impl TuiLoggerConfig {
    pub fn set_level(mut self, level: LevelFilter) -> Self {
        self.level = level;

        self
    }

    pub fn set_filter(mut self, filter: TargetFilter) -> Self {
        self.target_filter = filter;

        self
    }
}

#[derive(Debug)]
pub enum TargetFilter {
    Blacklist(Vec<String>),
    Whitelist(Vec<String>),
    None,
}

impl Default for TargetFilter {
    fn default() -> Self {
        Self::None
    }
}

impl TuiLogger {
    pub fn init(config: TuiLoggerConfig) -> Result<(), log::SetLoggerError> {
        set_max_level(config.level.clone());
        set_logger(&*TUI_LOGGER)?;

        (*TUI_LOGGER).conf.write().unwrap().level = config.level;
        (*TUI_LOGGER).conf.write().unwrap().target_filter = config.target_filter;

        Ok(())
    }
}

impl Log for TuiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if self.conf.read().unwrap().level >= metadata.level() {
            match &self.conf.read().unwrap().target_filter {
                TargetFilter::Blacklist(list) => {
                    !list.iter().any(|x| metadata.target().starts_with(x))
                }
                TargetFilter::Whitelist(list) => {
                    list.iter().any(|x| metadata.target().starts_with(x))
                }
                _ => true,
            }
        } else {
            return false;
        }
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.messages.write().unwrap().push(LogMessage {
                timestamp: Local::now(),
                file: record.file().unwrap_or_default().into(),
                level: record.level(),
                line: record.line().unwrap_or_default(),
                message: format!("{}", record.args()),
                target: record.target().into(),
            });
        }
    }

    fn flush(&self) {}
}

#[derive(Debug, Default)]
pub struct TuiLoggerWidget<'b> {
    block: Option<Block<'b>>,
    style: Style,
}

impl<'b> TuiLoggerWidget<'b> {
    const TRACE: Style = Style::new().fg(Color::White).modifier(Modifier::BOLD);
    const DEBUG: Style = Style::new().fg(Color::Cyan).modifier(Modifier::BOLD);
    const INFO: Style = Style::new().fg(Color::Blue).modifier(Modifier::BOLD);
    const WARN: Style = Style::new().fg(Color::Yellow).modifier(Modifier::BOLD);
    const ERROR: Style = Style::new().fg(Color::Red).modifier(Modifier::BOLD);

    const TIME: Style = Style::new().fg(Color::Gray);

    pub fn block(mut self, block: Block<'b>) -> TuiLoggerWidget<'b> {
        self.block = Some(block);
        self
    }
    pub fn style(mut self, style: Style) -> TuiLoggerWidget<'b> {
        self.style = style;
        self
    }
}

pub fn logger_text<'a>(height: u16) -> Vec<Text<'a>> {
    const TRACE: Style = Style::new().fg(Color::White).modifier(Modifier::BOLD);
    const DEBUG: Style = Style::new().fg(Color::Cyan).modifier(Modifier::BOLD);
    const INFO: Style = Style::new().fg(Color::Blue).modifier(Modifier::BOLD);
    const WARN: Style = Style::new().fg(Color::Yellow).modifier(Modifier::BOLD);
    const ERROR: Style = Style::new().fg(Color::Red).modifier(Modifier::BOLD);

    const TIME: Style = Style::new().fg(Color::Gray);

    (*TUI_LOGGER)
        .messages
        .write()
        .unwrap()
        .rev_iter()
        .flat_map(|message| {
            vec![
                Text::styled(message.timestamp.format("%H:%M:%S").to_string(), TIME),
                Text::styled(
                    message.level.to_string(),
                    match message.level {
                        Level::Trace => TRACE,
                        Level::Debug => DEBUG,
                        Level::Info => INFO,
                        Level::Warn => WARN,
                        Level::Error => ERROR,
                    },
                ),
            ]
        })
        .take(height as usize)
        .collect::<Vec<_>>()
}

fn write_wrap(
    text: String,
    area: Rect,
    buf: &mut Buffer,
    style: Style,
    offset: (u16, u16),
) -> (u16, u16) {
    let (ox, oy) = offset;

    let width = area.width - ox;
    let height = area.height - oy;

    let lines = text.len() as u16 / width - ox;
    let lines = lines + {
        if text.len() as u16 % width != 0 {
            1
        } else {
            0
        }
    };

    let mut chars = 0;
    for i in (0..lines).rev() {
        let start = (i * width) as usize;
        let end = min(start + width as usize, text.len());
        let text = &text[start..end];
        chars = text.len() as u16;

        let line = lines - i;
        let offset = line;

        if offset > height {
            return (chars, line);
        }

        buf.set_string(area.x, area.y + area.height - offset, text, style)
    }

    (chars, lines)
}

impl<'a> Widget for TuiLoggerWidget<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let area = match self.block {
            Some(ref mut b) => {
                b.render(area, buf);
                b.inner(area)
            }
            None => area,
        };

        if area.width < 8 || area.height < 1 {
            return;
        }

        buf.set_background(area, self.style.bg);

        let mut offset = 0;
        for message in (*TUI_LOGGER).messages.write().unwrap().rev_iter() {
            let (ox, oy) = write_wrap(
                format!("{:?}", message),
                area,
                buf,
                Default::default(),
                (0, offset),
            );

            offset += oy;

            if offset >= area.height {
                break;
            }
        }
    }
}
