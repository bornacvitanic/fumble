use crate::cli::tui::ui::LayoutSection;
use crate::cli::tui::widgets::bandwidth_widget::BandwidthWidget;
use crate::cli::tui::widgets::custom_widget::CustomWidget;
use crate::cli::tui::widgets::delay_widget::DelayWidget;
use crate::cli::tui::widgets::drop_widget::DropWidget;
use crate::cli::tui::widgets::duplicate_widget::DuplicateWidget;
use crate::cli::tui::widgets::filter_widget::FilterWidget;
use crate::cli::tui::widgets::logs_widget::LogsWidget;
use crate::cli::tui::widgets::reorder_widget::ReorderWidget;
use crate::cli::tui::widgets::tamper_widget::TamperWidget;
use crate::cli::tui::widgets::throttle_widget::ThrottleWidget;

pub struct TuiState<'a> {
    pub filter_widget: FilterWidget<'a>,
    pub sections:  Vec<CustomWidget<'a>>,
    pub logs_widget: LogsWidget,
    pub selected: usize,
    pub interacting: Option<usize>,
    pub focused: LayoutSection
}

impl<'a> TuiState<'a> {
    pub fn new() -> Self {
        TuiState {
            filter_widget: FilterWidget::new(),
            sections: vec![
                CustomWidget::Drop(DropWidget::new()),
                CustomWidget::Delay(DelayWidget::new()),
                CustomWidget::Throttle(ThrottleWidget::new()),
                CustomWidget::Reorder(ReorderWidget::new()),
                CustomWidget::Tamper(TamperWidget::new()),
                CustomWidget::Duplicate(DuplicateWidget::new()),
                CustomWidget::Bandwidth(BandwidthWidget::new()),
            ],
            selected: 0,
            interacting: None,
            logs_widget: LogsWidget::new(),
            focused: LayoutSection::Main
        }
    }
}