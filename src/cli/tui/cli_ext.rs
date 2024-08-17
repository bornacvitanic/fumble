use std::sync::{Arc, Mutex, RwLock};
use log::error;
use crate::cli::Cli;
use crate::cli::tui::state::TuiState;
use crate::cli::tui::traits::IsActive;
use crate::cli::tui::widgets::custom_widget::CustomWidget;
use crate::cli::tui::widgets::utils::{ParseFromTextArea, TextAreaExt};
use crate::network::modules::stats::PacketProcessingStatistics;
use crate::network::types::Probability;

pub trait TuiStateExt {
    /// Creates a `TuiState` instance from the current state of the `Cli` object.
    /// This function initializes the `TuiState` based on the settings stored in the `Cli`.
    fn from_cli(cli: &Arc<Mutex<Cli>>) -> Self;

    /// Updates the `TuiState` with the latest statistics from the packet processing.
    /// This function refreshes the widgets in the `TuiState` using data from the provided `PacketProcessingStatistics`.
    fn update_from(&mut self, statistics: &Arc<RwLock<PacketProcessingStatistics>>);
}

impl TuiStateExt for TuiState<'_> {
    fn from_cli(cli: &Arc<Mutex<Cli>>) -> Self {
        let mut state = TuiState::new();
        init_tui_state_from_cli(&mut state, &cli);
        state
    }

    fn update_from(&mut self, statistics: &Arc<RwLock<PacketProcessingStatistics>>) {
        update_tui_state_from_statistics(self, statistics);
    }
}

pub trait CliExt {
    /// Updates the `Cli` object based on the current state of the `TuiState`.
    /// This function applies the user inputs from the TUI to the `Cli`, synchronizing its settings with the interface state.
    fn update_from(&self, state: &mut TuiState);
}

impl CliExt for Arc<Mutex<Cli>> {
    fn update_from(&self, state: &mut TuiState) {
        update_cli_from_tui_state(state, &self);
    }
}

fn init_tui_state_from_cli(state: &mut TuiState, cli: &Arc<Mutex<Cli>>) {
    let cli = match cli.lock() {
        Ok(cli) => cli,
        Err(e) => {
            error!("Failed to lock CLI mutex. {}", e);
            return;
        }
    };

    if let Some(filter) = &cli.filter {
        state.filter_widget.textarea.set_text(filter);
    }
    for section in state.sections.iter_mut() {
        match section {
            CustomWidget::Drop(ref mut drop_widget) => {
                if let Some(probability) = cli.packet_manipulation_settings.drop.probability {
                    drop_widget.probability_text_area.set_text(&probability.value().to_string());
                    drop_widget.set_active(true);
                }
            }
            CustomWidget::Delay(ref mut delay_widget) => {
                if let Some(duration) = cli.packet_manipulation_settings.delay.duration {
                    delay_widget.delay_duration.set_text(&duration.to_string());
                    delay_widget.set_active(true);
                }
            }
            CustomWidget::Throttle(ref mut throttle_widget) => {
                if let Some(probability) = cli.packet_manipulation_settings.throttle.probability {
                    throttle_widget.probability_text_area.set_text(&probability.to_string());
                    throttle_widget.set_active(true);
                }
                throttle_widget.throttle_duration.set_text(&cli.packet_manipulation_settings.throttle.duration.to_string());
                throttle_widget.drop = cli.packet_manipulation_settings.throttle.drop;
            }
            CustomWidget::Reorder(ref mut reorder_widget) => {
                if let Some(duration) = cli.packet_manipulation_settings.reorder.max_delay {
                    reorder_widget.delay_duration.set_text(&duration.to_string());
                    reorder_widget.set_active(true);
                }
            }
            CustomWidget::Tamper(ref mut tamper_widget) => {
                if let Some(probability) = cli.packet_manipulation_settings.tamper.probability {
                    tamper_widget.probability_text_area.set_text(&probability.to_string());
                    tamper_widget.set_active(true);
                }
                tamper_widget.tamper_amount.set_text(&cli.packet_manipulation_settings.tamper.amount.to_string());
                if let Some(recalculate_checksums) = cli.packet_manipulation_settings.tamper.recalculate_checksums {
                    tamper_widget.recalculate_checksums = recalculate_checksums;
                }
            }
            CustomWidget::Duplicate(ref mut duplicate_widget) => {
                if let Some(probability) = cli.packet_manipulation_settings.duplicate.probability {
                    duplicate_widget.probability_text_area.set_text(&probability.to_string());
                    duplicate_widget.set_active(true);
                }
                duplicate_widget.duplicate_count.set_text(&cli.packet_manipulation_settings.duplicate.count.to_string());
            }
            CustomWidget::Bandwidth(ref mut bandwidth_widget) => {
                if let Some(duration) = cli.packet_manipulation_settings.bandwidth.limit {
                    bandwidth_widget.limit.set_text(&duration.to_string());
                    bandwidth_widget.set_active(true);
                }
            }
        }
    }
}

fn update_cli_from_tui_state(state: &mut TuiState, cli: &Arc<Mutex<Cli>>) {
    let mut cli = match cli.lock() {
        Ok(cli) => cli,
        Err(e) => {
            error!("Failed to lock CLI mutex. {}", e);
            return;
        }
    };

    for section in state.sections.iter_mut() {
        match section {
            CustomWidget::Drop(ref mut drop_widget) => {
                if !drop_widget.is_active() { cli.packet_manipulation_settings.drop.probability = None }
                else {
                    cli.packet_manipulation_settings.drop.probability = Probability::from_text_area(&drop_widget.probability_text_area);
                }
            }
            CustomWidget::Delay(ref mut delay_widget) => {
                if !delay_widget.is_active() { cli.packet_manipulation_settings.delay.duration = None }
                else {
                    cli.packet_manipulation_settings.delay.duration = u64::from_text_area(&delay_widget.delay_duration);
                }
            }
            CustomWidget::Throttle(ref mut throttle_widget) => {
                if !throttle_widget.is_active() { cli.packet_manipulation_settings.throttle.probability = None }
                else {
                    cli.packet_manipulation_settings.throttle.probability = Probability::from_text_area(&throttle_widget.probability_text_area);
                    if let Some(parsed_value) = u64::from_text_area(&throttle_widget.throttle_duration) {
                        cli.packet_manipulation_settings.throttle.duration = parsed_value;
                    }
                    cli.packet_manipulation_settings.throttle.drop = throttle_widget.drop;
                }
            }
            CustomWidget::Reorder(ref reorder_widget) => {
                if !reorder_widget.is_active() { cli.packet_manipulation_settings.reorder.max_delay = None }
                else {
                    cli.packet_manipulation_settings.reorder.max_delay = u64::from_text_area(&reorder_widget.delay_duration);
                }
            }
            CustomWidget::Tamper(ref tamper_widget) => {
                if !tamper_widget.is_active() { cli.packet_manipulation_settings.tamper.probability = None }
                else {
                    cli.packet_manipulation_settings.tamper.probability = Probability::from_text_area(&tamper_widget.probability_text_area);
                    if let Some(probability) = Probability::from_text_area(&tamper_widget.tamper_amount) {
                        cli.packet_manipulation_settings.tamper.amount = probability;
                    }
                    cli.packet_manipulation_settings.tamper.recalculate_checksums = Some(tamper_widget.recalculate_checksums);
                }
            }
            CustomWidget::Duplicate(ref duplicate_widget) => {
                if !duplicate_widget.is_active() { cli.packet_manipulation_settings.duplicate.probability = None }
                else {
                    cli.packet_manipulation_settings.duplicate.probability = Probability::from_text_area(&duplicate_widget.probability_text_area);
                    if let Some(parsed_value) = usize::from_text_area(&duplicate_widget.duplicate_count) {
                        cli.packet_manipulation_settings.duplicate.count = parsed_value;
                    }
                }
            }
            CustomWidget::Bandwidth(ref bandwidth_widget) => {
                if !bandwidth_widget.is_active() { cli.packet_manipulation_settings.bandwidth.limit = None }
                else {
                    cli.packet_manipulation_settings.bandwidth.limit = usize::from_text_area(&bandwidth_widget.limit);
                }
            }
        }
    }
}

fn update_tui_state_from_statistics(state: &mut TuiState, statistics: &Arc<RwLock<PacketProcessingStatistics>>) {
    let stats = match statistics.read() {
        Ok(stats) => stats,
        Err(e) => {
            error!("Failed to lock statistics read RwLock. {}", e);
            return;
        }
    };

    for section in state.sections.iter_mut() {
        if section.is_active() {
            match section {
                CustomWidget::Drop(ref mut drop_widget) => {drop_widget.update_data(&stats.drop_stats);}
                CustomWidget::Delay(ref mut delay_widget) => {delay_widget.update_data(&stats.delay_stats);}
                CustomWidget::Throttle(ref mut throttle_widget) => {throttle_widget.update_data(&stats.throttle_stats);}
                CustomWidget::Reorder(_) => {}
                CustomWidget::Tamper(_) => {}
                CustomWidget::Duplicate(_) => {}
                CustomWidget::Bandwidth(_) => {}
            }
        }
    }
}