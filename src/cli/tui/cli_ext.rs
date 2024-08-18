use std::sync::{Arc, Mutex, RwLock};
use log::error;
use crate::cli::Cli;
use crate::cli::settings::bandwidth::BandwidthOptions;
use crate::cli::settings::delay::DelayOptions;
use crate::cli::settings::drop::DropOptions;
use crate::cli::settings::duplicate::DuplicateOptions;
use crate::cli::settings::reorder::ReorderOptions;
use crate::cli::settings::tamper::TamperOptions;
use crate::cli::settings::throttle::ThrottleOptions;
use crate::cli::tui::state::TuiState;
use crate::cli::tui::traits::IsActive;
use crate::cli::tui::widgets::custom_widget::CustomWidget;
use crate::cli::tui::widgets::utils::{ParseFromTextArea, TextAreaExt};
use crate::network::modules::stats::PacketProcessingStatistics;
use crate::network::types::probability::Probability;

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
                if let Some(drop) = &cli.packet_manipulation_settings.drop {
                    drop_widget.probability_text_area.set_text(&drop.probability.value().to_string());
                    drop_widget.set_active(true);
                }
            }
            CustomWidget::Delay(ref mut delay_widget) => {
                if let Some(delay) = &cli.packet_manipulation_settings.delay {
                    delay_widget.delay_duration.set_text(&delay.duration.to_string());
                    delay_widget.set_active(true);
                }
            }
            CustomWidget::Throttle(ref mut throttle_widget) => {
                if let Some(throttle) = &cli.packet_manipulation_settings.throttle {
                    throttle_widget.probability_text_area.set_text(&throttle.probability.to_string());
                    throttle_widget.throttle_duration.set_text(&throttle.duration.to_string());
                    throttle_widget.drop = throttle.drop;
                    throttle_widget.set_active(true);
                }
            }
            CustomWidget::Reorder(ref mut reorder_widget) => {
                if let Some(reorder) = &cli.packet_manipulation_settings.reorder {
                    reorder_widget.delay_duration.set_text(&reorder.max_delay.to_string());
                    reorder_widget.set_active(true);
                }
            }
            CustomWidget::Tamper(ref mut tamper_widget) => {
                if let Some(tamper) = &cli.packet_manipulation_settings.tamper {
                    tamper_widget.probability_text_area.set_text(&tamper.probability.to_string());
                    tamper_widget.tamper_amount.set_text(&tamper.amount.to_string());
                    if let Some(recalculate_checksums) = tamper.recalculate_checksums {
                        tamper_widget.recalculate_checksums = recalculate_checksums;
                    }
                    tamper_widget.set_active(true);
                }
            }
            CustomWidget::Duplicate(ref mut duplicate_widget) => {
                if let Some(duplicate) = &cli.packet_manipulation_settings.duplicate {
                    duplicate_widget.probability_text_area.set_text(&duplicate.probability.to_string());
                    duplicate_widget.duplicate_count.set_text(&duplicate.count.to_string());
                    duplicate_widget.set_active(true);
                }
            }
            CustomWidget::Bandwidth(ref mut bandwidth_widget) => {
                if let Some(bandwidth) = &cli.packet_manipulation_settings.bandwidth {
                    bandwidth_widget.limit.set_text(&bandwidth.limit.to_string());
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
                cli.packet_manipulation_settings.drop = if !drop_widget.is_active() { None }
                else {
                    Probability::from_text_area(&drop_widget.probability_text_area)
                        .map(|probability| DropOptions { probability })
                }
            }
            CustomWidget::Delay(ref mut delay_widget) => {
                cli.packet_manipulation_settings.delay = if !delay_widget.is_active() { None }
                else {
                    u64::from_text_area(&delay_widget.delay_duration)
                        .map(|duration| DelayOptions { duration })
                }
            }
            CustomWidget::Throttle(ref mut throttle_widget) => {
                cli.packet_manipulation_settings.throttle = if !throttle_widget.is_active() { None }
                else {
                    Probability::from_text_area(&throttle_widget.probability_text_area)
                        .and_then(|probability| {
                            u64::from_text_area(&throttle_widget.throttle_duration)
                                .map(|duration| ThrottleOptions {
                                    probability,
                                    duration,
                                    drop: throttle_widget.drop,
                                })
                        })
                }
            }
            CustomWidget::Reorder(ref reorder_widget) => {
                cli.packet_manipulation_settings.reorder = if !reorder_widget.is_active() {
                    None
                } else {
                    Probability::from_text_area(&reorder_widget.probability_text_area)
                        .and_then(|probability| {
                            u64::from_text_area(&reorder_widget.delay_duration)
                                .map(|max_delay| ReorderOptions {
                                    probability,
                                    max_delay,
                                })
                        })
                }
            }

            CustomWidget::Tamper(ref tamper_widget) => {
                cli.packet_manipulation_settings.tamper = if !tamper_widget.is_active() { None }
                else {
                    Probability::from_text_area(&tamper_widget.probability_text_area)
                        .and_then(|probability| {
                            Probability::from_text_area(&tamper_widget.tamper_amount)
                                .map(|amount| TamperOptions {
                                    probability,
                                    amount,
                                    recalculate_checksums: Some(tamper_widget.recalculate_checksums),
                                })
                        })
                }
            }
            CustomWidget::Duplicate(ref duplicate_widget) => {
                cli.packet_manipulation_settings.duplicate = if !duplicate_widget.is_active() { None }
                else {
                    Probability::from_text_area(&duplicate_widget.probability_text_area)
                        .and_then(|probability| {
                            usize::from_text_area(&duplicate_widget.duplicate_count)
                                .map(|count| DuplicateOptions {
                                    probability,
                                    count,
                                })
                        })
                }
            }
            CustomWidget::Bandwidth(ref bandwidth_widget) => {
                cli.packet_manipulation_settings.bandwidth = if !bandwidth_widget.is_active() { None }
                else {
                    usize::from_text_area(&bandwidth_widget.limit)
                        .map(|limit| BandwidthOptions { limit })
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
                CustomWidget::Reorder(ref mut reorder_widget) => {reorder_widget.update_data(&stats.reorder_stats)}
                CustomWidget::Tamper(ref mut tamper_widget) => {tamper_widget.update_data(&stats.tamper_stats)}
                CustomWidget::Duplicate(ref mut duplicate_widget) => {duplicate_widget.update_data(&stats.duplicate_stats)}
                CustomWidget::Bandwidth(ref mut bandwidth_widget) => {bandwidth_widget.update_data(&stats.bandwidth_stats)}
            }
        }
    }
}