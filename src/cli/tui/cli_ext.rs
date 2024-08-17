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
    fn from_cli(cli: &Arc<Mutex<Cli>>) -> Self;
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
    fn update_from(&self, state: &mut TuiState);
}

impl CliExt for Arc<Mutex<Cli>> {
    fn update_from(&self, state: &mut TuiState) {
        update_cli_from_tui_state(state, &self);
    }
}

fn init_tui_state_from_cli(state: &mut TuiState, cli: &Arc<Mutex<Cli>>) {
    match cli.lock() {
        Ok(cli) => {
            if let Some(filter) = &cli.filter {
                state.filter_widget.textarea.set_text(filter);
            }
            if let CustomWidget::Drop(ref mut drop_widget) = state.sections[0] {
                if let Some(probability) = cli.packet_manipulation_settings.drop.probability {
                    drop_widget.probability_text_area.set_text(&probability.value().to_string());
                    drop_widget.set_active(true);
                }
            }
            if let CustomWidget::Delay(ref mut delay_widget) = state.sections[1] {
                if let Some(duration) = cli.packet_manipulation_settings.delay.duration {
                    delay_widget.delay_duration.set_text(&duration.to_string());
                    delay_widget.set_active(true);
                }
            }
            if let CustomWidget::Throttle(ref mut throttle_widget) = state.sections[2] {
                if let Some(probability) = cli.packet_manipulation_settings.throttle.probability {
                    throttle_widget.probability_text_area.set_text(&probability.to_string());
                    throttle_widget.set_active(true);
                }
                throttle_widget.throttle_duration.set_text(&cli.packet_manipulation_settings.throttle.duration.to_string());
                throttle_widget.drop = cli.packet_manipulation_settings.throttle.drop;
            }
            if let CustomWidget::Reorder(ref mut reorder_widget) = state.sections[3] {
                if let Some(duration) = cli.packet_manipulation_settings.reorder.max_delay {
                    reorder_widget.delay_duration.set_text(&duration.to_string());
                    reorder_widget.set_active(true);
                }
            }
            if let CustomWidget::Tamper(ref mut tamper_widget) = state.sections[4] {
                if let Some(probability) = cli.packet_manipulation_settings.tamper.probability {
                    tamper_widget.probability_text_area.set_text(&probability.to_string());
                    tamper_widget.set_active(true);
                }
                tamper_widget.tamper_amount.set_text(&cli.packet_manipulation_settings.tamper.amount.to_string());
                if let Some(recalculate_checksums) = cli.packet_manipulation_settings.tamper.recalculate_checksums {
                    tamper_widget.recalculate_checksums = recalculate_checksums;
                }
            }
            if let CustomWidget::Duplicate(ref mut duplicate_widget) = state.sections[5] {
                if let Some(probability) = cli.packet_manipulation_settings.duplicate.probability {
                    duplicate_widget.probability_text_area.set_text(&probability.to_string());
                    duplicate_widget.set_active(true);
                }
                duplicate_widget.duplicate_count.set_text(&cli.packet_manipulation_settings.duplicate.count.to_string());
            }
            if let CustomWidget::Bandwidth(ref mut bandwidth_widget) = state.sections[6] {
                if let Some(duration) = cli.packet_manipulation_settings.bandwidth.limit {
                    bandwidth_widget.limit.set_text(&duration.to_string());
                    bandwidth_widget.set_active(true);
                }
            }
        }
        Err(e) => {
            error!("Failed to lock CLI mutex. {}", e);
        }
    }
}

fn update_cli_from_tui_state(state: &mut TuiState, cli: &Arc<Mutex<Cli>>) {
    if let Ok(mut cli) = cli.lock() {
        if let CustomWidget::Drop(ref mut drop_widget) = state.sections[0] {
            if !drop_widget.is_active() { cli.packet_manipulation_settings.drop.probability = None }
            else {
                cli.packet_manipulation_settings.drop.probability = Probability::from_text_area(&drop_widget.probability_text_area);
            }
        }
        if let CustomWidget::Delay(ref mut delay_widget) = state.sections[1] {
            if !delay_widget.is_active() { cli.packet_manipulation_settings.delay.duration = None }
            else {
                cli.packet_manipulation_settings.delay.duration = u64::from_text_area(&delay_widget.delay_duration);
            }
        }
        if let CustomWidget::Throttle(ref mut throttle_widget) = state.sections[2] {
            if !throttle_widget.is_active() { cli.packet_manipulation_settings.throttle.probability = None }
            else {
                cli.packet_manipulation_settings.throttle.probability = Probability::from_text_area(&throttle_widget.probability_text_area);
                if let Some(parsed_value) = u64::from_text_area(&throttle_widget.throttle_duration) {
                    cli.packet_manipulation_settings.throttle.duration = parsed_value;
                }
                cli.packet_manipulation_settings.throttle.drop = throttle_widget.drop;
            }
        }
        if let CustomWidget::Reorder(ref reorder_widget) = state.sections[3] {
            if !reorder_widget.is_active() { cli.packet_manipulation_settings.reorder.max_delay = None }
            else {
                cli.packet_manipulation_settings.reorder.max_delay = u64::from_text_area(&reorder_widget.delay_duration);
            }
        }
        if let CustomWidget::Tamper(ref tamper_widget) = state.sections[4] {
            if !tamper_widget.is_active() { cli.packet_manipulation_settings.tamper.probability = None }
            else {
                cli.packet_manipulation_settings.tamper.probability = Probability::from_text_area(&tamper_widget.probability_text_area);
                if let Some(probability) = Probability::from_text_area(&tamper_widget.tamper_amount) {
                    cli.packet_manipulation_settings.tamper.amount = probability;
                }
                cli.packet_manipulation_settings.tamper.recalculate_checksums = Some(tamper_widget.recalculate_checksums);
            }
        }
        if let CustomWidget::Duplicate(ref duplicate_widget) = state.sections[5] {
            if !duplicate_widget.is_active() { cli.packet_manipulation_settings.duplicate.probability = None }
            else {
                cli.packet_manipulation_settings.duplicate.probability = Probability::from_text_area(&duplicate_widget.probability_text_area);
                if let Some(parsed_value) = usize::from_text_area(&duplicate_widget.duplicate_count) {
                    cli.packet_manipulation_settings.duplicate.count = parsed_value;
                }
            }
        }
        if let CustomWidget::Bandwidth(ref bandwidth_widget) = state.sections[6] {
            if !bandwidth_widget.is_active() { cli.packet_manipulation_settings.bandwidth.limit = None }
            else {
                cli.packet_manipulation_settings.bandwidth.limit = usize::from_text_area(&bandwidth_widget.limit);
            }
        }
    } else {
        // Handle the case where the mutex lock failed
        eprintln!("Failed to lock CLI mutex.");
    }
}

fn update_tui_state_from_statistics(state: &mut TuiState, statistics: &Arc<RwLock<PacketProcessingStatistics>>) {
    match statistics.read() {
        Ok(stats) => {
            if let CustomWidget::Drop(ref mut drop_widget) = state.sections[0] {
                if drop_widget.is_active() {
                    drop_widget.update_data(&stats.drop_stats);
                }
            }
            if let CustomWidget::Delay(ref mut delay_widget) = state.sections[1] {
                if delay_widget.is_active() {
                    delay_widget.update_data(&stats.delay_stats);
                }
            }
            if let CustomWidget::Throttle(ref mut throttle_widget) = state.sections[2] {
                if throttle_widget.is_active() {
                    throttle_widget.update_data(&stats.throttle_stats);
                }
            }
        }
        Err(e) => {
            error!("Failed to lock statistics read RwLock. {}", e);
        }
    }
}