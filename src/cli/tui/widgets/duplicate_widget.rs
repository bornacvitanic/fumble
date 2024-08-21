use crate::cli::tui::traits::{DisplayName, HandleInput, IsActive, KeyBindings};
use crate::cli::tui::widgets::utils::block_ext::RoundedBlockExt;
use crate::cli::tui::widgets::utils::style_textarea_based_on_validation;
use crate::cli::tui::widgets::utils::textarea_ext::TextAreaExt;
use crate::cli::tui::widgets::utils::textarea_parsing::ParseFromTextArea;
use crate::network::modules::stats::duplicate_stats::DuplicateStats;
use crate::network::types::probability::Probability;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph, Widget};
use tui_textarea::TextArea;

pub struct DuplicateWidget<'a> {
    title: String,
    probability_text_area: TextArea<'a>,
    duplicate_count_text_area: TextArea<'a>,
    is_active: bool,
    interacting: bool,
    pub probability: Result<Probability, String>,
    pub duplicate_count: Result<usize, String>,
    selected: usize,
    duplication_multiplier: f64,
}

impl Default for DuplicateWidget<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl DuplicateWidget<'_> {
    pub fn new() -> Self {
        DuplicateWidget {
            title: "Duplicate".to_string(),
            probability_text_area: TextArea::default(),
            duplicate_count_text_area: TextArea::default(),
            is_active: false,
            interacting: false,
            probability: Ok(Probability::default()),
            duplicate_count: Ok(1),
            selected: 0,
            duplication_multiplier: 1.0,
        }
    }

    pub fn set_probability(&mut self, probability: Probability) {
        self.probability_text_area
            .set_text(&probability.to_string());
        self.probability = Ok(probability);
    }

    pub fn set_duplicate_count(&mut self, duplicate_count: usize) {
        self.duplicate_count_text_area
            .set_text(&duplicate_count.to_string());
        self.duplicate_count = Ok(duplicate_count);
    }

    pub(crate) fn update_data(&mut self, stats: &DuplicateStats) {
        self.duplication_multiplier = stats.recent_duplication_multiplier();
    }
}

impl HandleInput for DuplicateWidget<'_> {
    fn handle_input(&mut self, key: KeyEvent) -> bool {
        if !self.interacting {
            if key.code == KeyCode::Enter && key.kind == KeyEventKind::Press {
                self.interacting = true;
                return true;
            }
        } else {
            if let KeyCode::Enter | KeyCode::Esc = key.code {
                self.interacting = false;
                return false;
            }
            if key.code == KeyCode::Down && self.selected < 1 {
                self.selected += 1;
            }
            if key.code == KeyCode::Up && self.selected > 0 {
                self.selected -= 1;
            }
            match self.selected {
                0 => {
                    if self.probability_text_area.input(key) {
                        self.probability =
                            Probability::parse_from_text_area(&self.probability_text_area);
                    }
                }
                1 => {
                    if self.duplicate_count_text_area.input(key) {
                        self.duplicate_count =
                            usize::parse_from_text_area(&self.duplicate_count_text_area);
                    }
                }
                _ => {}
            }

            return true;
        }
        false
    }
}

impl DisplayName for DuplicateWidget<'_> {
    fn name(&self) -> &str {
        &self.title
    }
}

impl KeyBindings for DuplicateWidget<'_> {
    fn key_bindings(&self) -> String {
        "Exit: Esc | Navigation: Up and Down".to_string()
    }
}

impl IsActive for DuplicateWidget<'_> {
    fn is_active(&self) -> bool {
        self.is_active
    }

    fn set_active(&mut self, state: bool) {
        self.is_active = state;
    }
}

impl Widget for &mut DuplicateWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [probability_area, duration_area, info_area] = Layout::horizontal([
            Constraint::Max(10),
            Constraint::Max(10),
            Constraint::Min(25),
        ])
        .areas(area.inner(Margin {
            horizontal: 1,
            vertical: 1,
        }));

        self.probability_text_area
            .set_cursor_visibility(self.interacting && self.selected == 0);
        self.probability_text_area.set_dim_placeholder("0.1");
        self.probability_text_area
            .set_cursor_line_style(Style::default());
        self.probability_text_area.set_block(
            Block::roundedt("Probability").highlight_if(self.interacting && self.selected == 0),
        );
        if !self.probability_text_area.lines()[0].is_empty() {
            style_textarea_based_on_validation(&mut self.probability_text_area, &self.probability);
        }
        self.probability_text_area.render(probability_area, buf);

        self.duplicate_count_text_area
            .set_cursor_visibility(self.interacting && self.selected == 1);
        self.duplicate_count_text_area.set_dim_placeholder("1");
        self.duplicate_count_text_area
            .set_cursor_line_style(Style::default());
        self.duplicate_count_text_area.set_block(
            Block::roundedt("Count").highlight_if(self.interacting && self.selected == 1),
        );
        if !self.duplicate_count_text_area.lines()[0].is_empty() {
            style_textarea_based_on_validation(
                &mut self.duplicate_count_text_area,
                &self.duplicate_count,
            );
        }
        self.duplicate_count_text_area.render(duration_area, buf);

        let [duplication_multiplier_info, _excess_info] =
            Layout::horizontal([Constraint::Max(20), Constraint::Fill(1)]).areas(info_area);
        Paragraph::new(format!("{:.2}x", self.duplication_multiplier))
            .block(Block::bordered().title("I/O Multiplier"))
            .render(duplication_multiplier_info, buf);
    }
}