use std::{fmt::Debug, time::Duration};

use anyhow::Result;
use api::characters::models::Character;
use api::compatibility::FromLegacy;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use openapi::{
    apis::{Api, ApiClient, configuration::Configuration},
    models::MyAccountDetailsSchema,
};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Flex, Rect},
    prelude::{Constraint, Layout, Span},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph, Widget},
};
use ratatui_macros::{constraint, constraints, horizontal, vertical};
use std::io;
use tokio::time::sleep;
use tracing::{error, info};
use widgets::character_info::CharacterInfo;
mod api;
mod widgets;

/// A generic retry helper that retries an async operation with exponential backoff.
///
/// - `operation`: A closure that returns a Future which outputs a `Result<T, E>`.
/// - `max_retries`: Maximum number of attempts before giving up.
/// - `initial_delay`: The starting delay duration between attempts.
///
/// Returns the successful value or the last error.
async fn retry<T, E, Fut, F>(
    mut operation: F,
    max_retries: usize,
    initial_delay: Duration,
) -> Result<T, E>
where
    E: Debug,
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut attempt = 0;
    let mut delay = initial_delay;

    loop {
        match operation().await {
            Ok(value) => return Ok(value),
            Err(e) => {
                attempt += 1;
                if attempt > max_retries {
                    error!("Operation failed after {} attempts: {:?}", attempt, e);
                    return Err(e);
                } else {
                    error!(
                        "Attempt {} failed: {:?}. Retrying in {} seconds...",
                        attempt,
                        e,
                        delay.as_secs()
                    );
                    sleep(delay).await;
                    delay *= 2;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().map_err(|e| anyhow::anyhow!(e))?;

    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .pretty()
        .with_writer(non_blocking)
        .init();

    dotenv::dotenv().ok();
    let Ok(artifacts_api_key) = dotenv::var("ARTIFACTS_API_KEY") else {
        error!("ARTIFACTS_API_KEY is not set");
        std::process::exit(1);
    };

    info!("ARTIFACTS_API_KEY is set.");

    let mut cfg = Configuration::new();
    cfg.bearer_access_token = Some(artifacts_api_key);

    let client = ApiClient::new(cfg.into());

    let mut terminal = ratatui::init();
    let result = App::new(client).run_async(&mut terminal).await;
    ratatui::restore();

    result.map_err(|e| anyhow::anyhow!(e))
}

async fn get_account(client: &ApiClient) -> anyhow::Result<MyAccountDetailsSchema> {
    retry(
        || async {
            client
                .my_account_api()
                .get_account_details_my_details_get()
                .await
        },
        3,
        Duration::from_secs(1),
    )
    .await
    .map_err(|e| anyhow::anyhow!(e))
}

async fn get_characters(client: &ApiClient) -> Result<Vec<Character>> {
    retry(
        || async {
            client
                .my_characters_api()
                .get_my_characters_my_characters_get()
                .await
        },
        3,
        Duration::from_secs(1),
    )
    .await
    .map(|itm| itm.data.into_iter().map(Character::from_legacy).collect())
    .map_err(|e| anyhow::anyhow!(e))
}

pub struct App {
    client: ApiClient,
    exit: bool,
    account: MyAccountDetailsSchema,
    characters: Vec<Box<Character>>,
}

impl App {
    pub fn new(client: ApiClient) -> Self {
        Self {
            client,
            exit: false,
            account: MyAccountDetailsSchema::default(),
            characters: Vec::new(),
        }
    }

    pub async fn run_async(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let account = get_account(&self.client);
        let characters = get_characters(&self.client);

        let (account, characters) = tokio::try_join!(account, characters)?;
        self.account = account;
        self.characters = characters.into_iter().map(Box::new).collect();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            // TODO: Add more key bindings here
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Artifacts MMO CLI ".bold());
        let block = Block::bordered()
            .title_top(title.centered())
            .title_bottom(" Press 'q' to exit ")
            .padding(Padding::uniform(1))
            .border_set(border::THICK);

        let [_, widget_layout] = vertical![*=0, >=0].flex(Flex::SpaceAround).areas(area);

        let [account_area, characters_area] = vertical![==3, ==100%]
            .spacing(1)
            .areas(block.inner(widget_layout));

        block.render(widget_layout, buf);
        self.render_account_info(buf, account_area);
        self.render_characters_info(buf, characters_area);
    }
}

impl App {
    fn render_account_info(&self, buf: &mut Buffer, area: Rect) {
        let acct_block = Block::default()
            .padding(Padding::left(4))
            .title(" Account Details:".bold());

        let body = Text::from(vec![
            Line::from(vec![
                "Username: ".bold(),
                Span::raw(self.account.data.username.clone()),
            ]),
            Line::from(vec![
                "Email: ".bold(),
                Span::raw(self.account.data.email.clone()),
            ]),
        ]);

        Paragraph::new(body).block(acct_block).render(area, buf);
    }

    fn render_characters_info(&self, buf: &mut Buffer, area: Rect) {
        let num_widgets = self.characters.len();
        let min_width: u16 = 30;
        let max_width: u16 = 100;

        // Determine maximum columns such that each widget is at least `min_width`
        // (ensure at least one column)
        let mut columns = if area.width >= min_width {
            area.width / min_width
        } else {
            1
        };
        if columns == 0 {
            columns = 1;
        }

        // Calculate what the widget width would be if using this many columns
        let mut widget_width = area.width / columns;

        // Reduce column count until each widget's width does not exceed `max_width`
        while widget_width > max_width && columns > 1 {
            columns -= 1;
            widget_width = area.width / columns;
        }
        // In case a single widget would otherwise exceed max_width, clamp it
        if widget_width > max_width {
            widget_width = max_width;
        }

        // Calculate the required number of rows
        let rows = (num_widgets as f64 / columns as f64).ceil() as u16;

        // Create horizontal constraints for each column with fixed widget width.
        let horizontal_constraints = vec![Constraint::Length(widget_width); columns as usize];
        // Assuming a fixed widget height (as in your original code, here 7)
        let widget_height: u16 = 7;
        let vertical_constraints = vec![Constraint::Length(widget_height); rows as usize];

        // Split the main area vertically into rows.
        let row_areas = Layout::vertical(vertical_constraints).split(area);

        for (i, character) in self.characters.iter().enumerate() {
            let row = (i as u16) / columns;
            let col = (i as u16) % columns;

            // For each row, split horizontally into the computed columns.
            let col_areas =
                Layout::horizontal(horizontal_constraints.clone()).split(row_areas[row as usize]);
            let widget_area = col_areas[col as usize];

            CharacterInfo::new(character.clone()).render(widget_area, buf);
        }
    }
}
