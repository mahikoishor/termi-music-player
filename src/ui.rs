use crate::player::{PlayerState, TermiPlayer};
use arboard::Clipboard;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::{io, time::Duration};
use tui::{
    Terminal,
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
};

pub enum EventHandlerResponse {
    Continue,
    Break,
    None,
}

#[derive(Default)]
pub struct TermiUi {
    player: TermiPlayer,
    path_input: String, // For file/dir input when Empty
}

impl TermiUi {
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        self.player.prepare();

        loop {
            
            terminal.draw(|frame| {
                let size = frame.size();

                if self.player.state == PlayerState::Empty {
                    let area = centered_rect(60, 20, size);

                    frame.render_widget(self.input_box(), area);
                } else {
                    let layout = self.get_layout(size);

                    frame.render_widget(self.header(), layout[0]);
                    // layout[1] is the flexible gap, nothing rendered
                    frame.render_widget(self.timeline(), layout[2]);
                    frame.render_widget(self.controls(), layout[3]);
                    frame.render_widget(self.statusbar(), layout[4]);
                }
            })?;

            if crossterm::event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind != crossterm::event::KeyEventKind::Press {
                        continue;
                    }

                    let response = if self.player.state == PlayerState::Empty {
                        self.event_handler_empty_state(key)
                    } else {
                        self.event_handler_main_state(key)
                    };

                    match response {
                        EventHandlerResponse::Break => {
                            break Ok(());
                        }
                        EventHandlerResponse::Continue => {
                            continue;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn event_handler_empty_state(&mut self, key: KeyEvent) -> EventHandlerResponse {
        match key.code {
            KeyCode::Char('v') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                // Paste from clipboard
                if let Ok(mut clipboard) = Clipboard::new() {
                    if let Ok(text) = clipboard.get_text() {
                        self.path_input = text;
                    }
                }

                return EventHandlerResponse::Continue;
            }
            KeyCode::Char(c)
                if key.modifiers.is_empty() || key.modifiers == event::KeyModifiers::SHIFT =>
            {
                self.path_input.push(c);
            }
            KeyCode::Backspace => {
                self.path_input.pop();
            }
            KeyCode::Enter => {
                self.player.open(&self.path_input);
                self.path_input.clear();
            }
            KeyCode::Esc => {
                return EventHandlerResponse::Break;
            }
            _ => {}
        }

        EventHandlerResponse::None
    }

    fn event_handler_main_state(&mut self, key: KeyEvent) -> EventHandlerResponse {
        match key.code {
            KeyCode::Esc => {
                return EventHandlerResponse::Break;
            }
            KeyCode::Char('o') => {
                self.player.pause();
                self.player.state = PlayerState::Empty;
            }
            KeyCode::Char(' ') => {
                self.player.toggle_play();
            }
            KeyCode::Left => {
                self.player.previous();
            }
            KeyCode::Right => {
                self.player.next();
            }
            KeyCode::Up => {
                self.player.volume_up();
            }
            KeyCode::Down => {
                self.player.volume_down();
            }
            _ => {}
        }

        EventHandlerResponse::None
    }

    fn get_layout(&self, size: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3), // Header
                    Constraint::Min(0),    // Flexible gap
                    Constraint::Length(3), // Timeline
                    Constraint::Length(3), // Controls
                    Constraint::Length(3), // Statusbar
                ]
                .as_ref(),
            )
            .split(size)
    }

    fn header(&self) -> Paragraph {
        let music = &self.player.music;

        Paragraph::new(format!("{}", music.title))
            .block(Block::default().borders(Borders::ALL).title("Title"))
            .alignment(Alignment::Left)
    }

    fn timeline(&self) -> Gauge {
        let (current_time, total_time) = self.player.current_position();

        let ratio = if total_time > 0 {
            current_time as f64 / total_time as f64
        } else {
            0.0
        };

        // Convert seconds to MM:SS format
        let current_minutes = current_time / 60;
        let current_seconds = current_time % 60;
        let total_minutes = total_time / 60;
        let total_seconds = total_time % 60;

        Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Timeline"))
            .gauge_style(Style::default().add_modifier(Modifier::BOLD))
            .ratio(ratio.min(1.0))
            .label(format!(
                "{:02}:{:02}/{:02}:{:02}    Volume: {:02}%",
                current_minutes,
                current_seconds,
                total_minutes,
                total_seconds,
                self.player.engine.volume as u32
            ))
    }

    fn controls(&self) -> Paragraph {
        let playing_or_pause = if self.player.state == PlayerState::Play {
            "Pause"
        } else {
            "Play "
        };

        Paragraph::new(format!("< Previous | Next > | SPACE {playing_or_pause} | ↑↓ Vol | o Open | ESC Exit"))
            .block(Block::default().borders(Borders::ALL).title("Controls"))
            .alignment(Alignment::Left)
    }

    fn statusbar(&self) -> Paragraph {
        Paragraph::new(format!(
            "Playlist: {}/{} | Message: {}",
            self.player.current_index,
            self.player.playlist.len(),
            self.player.status.as_ref().unwrap_or(&String::from("None"))
        ))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Right)
    }

    fn input_box(&self) -> Paragraph {
        Paragraph::new(format!(
            "Enter file or directory path:\n{}",
            self.path_input
        ))
        .block(Block::default().borders(Borders::ALL).title("Open Path"))
        .alignment(Alignment::Left)
    }
}

// Helper function to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    let vertical = popup_layout[1];
    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical);
    horizontal_layout[1]
}
