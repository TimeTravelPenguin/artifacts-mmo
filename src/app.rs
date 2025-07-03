use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::{DefaultTerminal, prelude::*};
use tokio::task;
use tracing::{error, info};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiWidgetEvent, TuiWidgetState};

use crate::api::client::ArtifactsClient;

pub struct App {
    client: ArtifactsClient,
    app_state: TuiWidgetState,
    running: bool,
    event_stream: EventStream,
}

impl App {
    pub fn new(client: ArtifactsClient) -> Self {
        Self {
            client,
            running: false,
            event_stream: EventStream::new(),
            app_state: TuiWidgetState::new()
                .set_default_display_level(tui_logger::LevelFilter::Info),
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        self.running = true;

        let client = self.client.clone();

        task::spawn(async move {
            if let Err(e) = make_request(client).await {
                error!(target: "App", "Error making request: {:?}", e);
            }
        });

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
        let state = &mut self.app_state;

        match key.code {
            KeyCode::Char('q') => self.quit(),
            KeyCode::Char('c') | KeyCode::Char('C') if key.modifiers == KeyModifiers::CONTROL => {
                self.quit();
            }
            KeyCode::Char(' ') => state.transition(TuiWidgetEvent::SpaceKey),
            KeyCode::Esc => state.transition(TuiWidgetEvent::EscapeKey),
            KeyCode::PageUp => state.transition(TuiWidgetEvent::PrevPageKey),
            KeyCode::PageDown => state.transition(TuiWidgetEvent::NextPageKey),
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
        let [log_area, help_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(area);

        TuiLoggerSmartWidget::default()
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Magenta))
            .style_info(Style::default().fg(Color::Cyan))
            .output_level(Some(TuiLoggerLevelOutput::Long))
            .state(&self.app_state)
            .render(log_area, buf);

        if area.width > 40 {
            Text::from(vec![
                "Q: Quit | Tab: Switch state | ↑/↓: Select target | f: Focus target".into(),
                "←/→: Display level | +/-: Filter level | Space: Toggle hidden targets".into(),
                "h: Hide target selector | PageUp/Down: Scroll | Esc: Cancel scroll".into(),
            ])
            .style(Color::Gray)
            .centered()
            .render(help_area, buf);
        }
    }
}

async fn make_request(api: ArtifactsClient) -> anyhow::Result<()> {
    let character_name = "Penguin";
    let character = api.get_character(character_name).await?;

    info!(target:"make_request", "Character: {:?}", character);

    Ok(())
}
