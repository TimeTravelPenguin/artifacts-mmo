use ratatui::{symbols::border, widgets::Gauge};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize, palette::tailwind},
    widgets::{Block, Padding, Paragraph, Widget},
};
use ratatui_macros::{horizontal, vertical};

use crate::api::characters::models::Character;

pub struct CharacterInfo {
    character: Box<Character>,
}

#[derive(Debug)]
enum GaugeType {
    HP,
    XP,
}

struct GaugeInfo {
    gauge_type: GaugeType,
    max: f64,
    current: f64,
}

impl GaugeInfo {
    fn hp(character: &Character) -> Self {
        Self {
            gauge_type: GaugeType::HP,
            max: character.stats.max_hp as f64,
            current: character.stats.hp as f64,
        }
    }

    fn xp(character: &Character) -> Self {
        Self {
            gauge_type: GaugeType::XP,
            max: character.max_xp as f64,
            current: character.xp as f64,
        }
    }

    fn progress(&self) -> f64 {
        self.current / self.max
    }

    fn label(&self) -> String {
        format!("{:?}", self.gauge_type)
    }

    fn progress_ratio_as_string(&self) -> String {
        format!("{}/{}", self.current, self.max)
    }

    fn style(&self) -> Style {
        match self.gauge_type {
            GaugeType::XP { .. } => Style::new().fg(tailwind::BLUE.c600).bg(tailwind::BLUE.c200),
            GaugeType::HP { .. } => {
                let progress = self.progress();
                if progress <= 0.2 {
                    Style::new().fg(tailwind::RED.c600).bg(tailwind::RED.c200)
                } else if progress <= 0.5 {
                    Style::new()
                        .fg(tailwind::YELLOW.c600)
                        .bg(tailwind::YELLOW.c200)
                } else {
                    Style::new()
                        .fg(tailwind::GREEN.c600)
                        .bg(tailwind::GREEN.c200)
                }
            }
        }
    }
}

impl Widget for &GaugeInfo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let label = self.progress_ratio_as_string();

        let [label_area, gauge_area] = horizontal![==2, >=2 + label.len() as u16]
            .spacing(1)
            .areas(area);

        Paragraph::new(self.label().bold()).render(label_area, buf);

        Gauge::default()
            .gauge_style(self.style())
            .label(label)
            .ratio(self.progress())
            .use_unicode(true)
            .render(gauge_area, buf);
    }
}

impl CharacterInfo {
    pub fn new(character: Box<Character>) -> Self {
        Self { character }
    }
}

impl Widget for &CharacterInfo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = format!(
            " Lv {}. {} ",
            self.character.level,
            self.character.name.clone()
        )
        .bold();

        let block = Block::bordered()
            .border_set(border::ROUNDED)
            .title(title)
            .padding(Padding::symmetric(4, 1));

        let [gauge_hp, gauge_xp] = vertical![==1; 2].spacing(1).areas(block.inner(area));

        block.render(area, buf);

        GaugeInfo::hp(&self.character).render(gauge_hp, buf);
        GaugeInfo::xp(&self.character).render(gauge_xp, buf);
    }
}
