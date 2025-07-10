use std::sync::{Arc, Mutex};

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::{
    prelude::*,
    widgets::{Gauge, Paragraph},
};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiWidgetEvent, TuiWidgetState};

use crate::api::client::ArtifactsClient;
use crate::models::character::Character;

#[derive(Debug, Clone, Default)]
pub struct CharacterWidgetState {
    pub characters: Vec<Character>,
    pub direction: Direction,
}

impl CharacterWidgetState {
    pub fn characters(mut self, characters: Vec<Character>) -> Self {
        self.characters = characters;
        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
}

impl CharacterWidgetState {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default)]
pub struct CharacterWidget;

impl CharacterWidget {
    pub fn new() -> Self {
        Self
    }
}

impl StatefulWidget for &CharacterWidget {
    type State = CharacterWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let characters = state.characters.clone();
        let num_chars = characters.len();

        if num_chars == 0 {
            Paragraph::new("No characters available")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray))
                .render(area, buf);
            return;
        }

        let constraints = std::iter::repeat_n(Constraint::Fill(1), num_chars).collect::<Vec<_>>();

        let areas = if state.direction == Direction::Horizontal {
            Layout::horizontal(constraints).split(area)
        } else {
            Layout::vertical(constraints).split(area)
        };

        for (char, area) in characters.iter().zip(areas.iter()) {
            let [name_area, hp_bar_area, xp_bar_area] = Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .areas(*area);

            let level_text = format!("Lv. {} {}", char.level, char.name);
            let [name_area, cooldown_area] = Layout::horizontal([
                Constraint::Length(level_text.len() as u16 + 2),
                Constraint::Fill(1),
            ])
            .areas(name_area);

            Paragraph::new(Text::from(format!("Lv. {} {}", char.level, char.name)))
                .style(Style::default().fg(Color::White))
                .render(name_area, buf);

            let remaining_sec = {
                let now = chrono::Utc::now();
                let remaining = char.cooldown_expiration.signed_duration_since(now);
                remaining.num_seconds().max(0)
            };

            if remaining_sec > 0 {
                Paragraph::new(Text::from(format!("(Cooldown: {}s)", remaining_sec as u32)))
                    .style(Style::default().fg(Color::Yellow))
                    .render(cooldown_area, buf);
            }

            // HP bar
            let [hp_label_area, hp_bar_area] =
                Layout::horizontal([Constraint::Length(12), Constraint::Max(20)])
                    .areas(hp_bar_area);

            Paragraph::new(Text::from(format!("HP: {}/{}", char.hp, char.max_hp)))
                .render(hp_label_area, buf);

            let hp_ratio = if char.max_hp > 0 {
                char.hp as f64 / char.max_hp as f64
            } else {
                0.0
            };

            let hp_percentage = (100.0 * hp_ratio).round() as u16;

            let color = if hp_percentage < 20 {
                Color::Red
            } else if hp_percentage < 50 {
                Color::Yellow
            } else {
                Color::Green
            };

            Gauge::default()
                .gauge_style(Style::default().fg(color).bg(Color::Black))
                .ratio(hp_ratio)
                .label(format!("{}%", hp_percentage))
                .render(hp_bar_area, buf);

            // XP bar
            let [xp_label_area, xp_bar_area] =
                Layout::horizontal([Constraint::Length(12), Constraint::Max(20)])
                    .areas(xp_bar_area);

            Paragraph::new(Text::from(format!("XP: {}/{}", char.xp, char.max_xp)))
                .render(xp_label_area, buf);

            let xp_ratio = if char.max_xp > 0 {
                char.xp as f64 / char.max_xp as f64
            } else {
                0.0
            };

            let xp_percentage = (100.0 * xp_ratio).round() as u16;

            Gauge::default()
                .gauge_style(Style::default().fg(Color::Blue).bg(Color::Black))
                .ratio(xp_ratio)
                .label(format!("{}%", xp_percentage))
                .render(xp_bar_area, buf);
        }
    }
}

pub struct App {
    client: ArtifactsClient,
    running: bool,
    event_stream: EventStream,
    tui_widget_state: TuiWidgetState,
    character_widget: Arc<Mutex<CharacterWidgetState>>,
}

impl App {
    pub fn new(client: ArtifactsClient) -> Self {
        Self {
            client,
            running: false,
            event_stream: EventStream::new(),
            tui_widget_state: TuiWidgetState::new()
                .set_default_display_level(tui_logger::LevelFilter::Info),
            character_widget: Arc::new(Mutex::new(CharacterWidgetState::new())),
        }
    }

    pub fn character_widget_state(&self) -> Arc<Mutex<CharacterWidgetState>> {
        self.character_widget.clone()
    }

    pub async fn run<B: Backend>(mut self, mut terminal: Terminal<B>) -> anyhow::Result<()> {
        self.running = true;

        while self.running {
            terminal.draw(|f| {
                f.render_widget(&self, f.area());
            })?;
            self.handle_crossterm_events().await?;
        }

        Ok(())
    }

    async fn handle_crossterm_events(&mut self) -> anyhow::Result<()> {
        tokio::select! {
            event = self.event_stream.next().fuse() => {
                if let Some(Ok(evt)) = event {
                    match evt {
                        Event::Key(key)
                            if key.kind == KeyEventKind::Press
                                => self.on_key_event(key),
                        Event::Mouse(_) => {}
                        Event::Resize(_, _) => {}
                        _ => {}
                    }
                }
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                // Sleep for a short duration to avoid busy waiting.
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        let state = &mut self.tui_widget_state;

        match key.code {
            KeyCode::Char('q') => self.quit(),
            KeyCode::Char('c') | KeyCode::Char('C') if key.modifiers == KeyModifiers::CONTROL => {
                self.quit();
            }
            KeyCode::Char(' ') => state.transition(TuiWidgetEvent::SpaceKey),
            KeyCode::Esc => state.transition(TuiWidgetEvent::EscapeKey),
            KeyCode::Up if key.modifiers == KeyModifiers::SHIFT => {
                state.transition(TuiWidgetEvent::PrevPageKey)
            }
            KeyCode::Down if key.modifiers == KeyModifiers::SHIFT => {
                state.transition(TuiWidgetEvent::NextPageKey)
            }
            KeyCode::Up => state.transition(TuiWidgetEvent::UpKey),
            KeyCode::Down => state.transition(TuiWidgetEvent::DownKey),
            KeyCode::Left => state.transition(TuiWidgetEvent::LeftKey),
            KeyCode::Right => state.transition(TuiWidgetEvent::RightKey),
            KeyCode::Char('+') => state.transition(TuiWidgetEvent::PlusKey),
            KeyCode::Char('-') => state.transition(TuiWidgetEvent::MinusKey),
            KeyCode::Char('h') => state.transition(TuiWidgetEvent::HideKey),
            KeyCode::Char('f') => state.transition(TuiWidgetEvent::FocusKey),
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [app_area, log_area] = if area.height >= 40 {
            Layout::vertical([Constraint::Percentage(20), Constraint::Fill(1)]).areas(area)
        } else {
            Layout::horizontal([Constraint::Max(40), Constraint::Fill(1)]).areas(area)
        };

        if let Ok(mut state) = self.character_widget.lock() {
            state.direction = if app_area.height >= 40 {
                Direction::Vertical
            } else {
                Direction::Horizontal
            };

            let area = app_area.inner(Margin::new(1, 1));
            CharacterWidget::new().render(area, buf, &mut state);
        }

        let [log_area, help_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(log_area);

        TuiLoggerSmartWidget::default()
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Magenta))
            .style_info(Style::default().fg(Color::Cyan))
            .output_level(Some(TuiLoggerLevelOutput::Long))
            .output_file(false)
            .output_line(false)
            .state(&self.tui_widget_state)
            .render(log_area, buf);

        if area.width >= 140 {
            Text::from(vec![
                "Q: Quit | Tab: Switch state | ↑/↓: Select target | f: Focus target".into(),
                "←/→: Display level | +/-: Filter level | Space: Toggle hidden targets".into(),
                "h: Hide target selector | Shift + ↑/↓: Scroll | Esc: Cancel scroll".into(),
            ])
            .style(Color::Gray)
            .centered()
            .render(help_area, buf);
        }
    }
}
