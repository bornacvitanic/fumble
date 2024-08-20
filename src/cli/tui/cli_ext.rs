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
use crate::cli::Cli;
use crate::network::modules::stats::PacketProcessingStatistics;
use log::error;
use std::sync::{Arc, Mutex, RwLock};

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
        init_tui_state_from_cli(&mut state, cli);
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

    fn clear_state(&self);
}

impl CliExt for Arc<Mutex<Cli>> {
    fn update_from(&self, state: &mut TuiState) {
        update_cli_from_tui_state(state, self);
    }

    fn clear_state(&self) {
        let mut cli = match self.lock() {
            Ok(cli) => cli,
            Err(e) => {
                error!("Failed to lock CLI mutex. {}", e);
                return;
            }
        };

        cli.packet_manipulation_settings.drop = None;
        cli.packet_manipulation_settings.delay = None;
        cli.packet_manipulation_settings.throttle = None;
        cli.packet_manipulation_settings.reorder = None;
        cli.packet_manipulation_settings.tamper = None;
        cli.packet_manipulation_settings.duplicate = None;
        cli.packet_manipulation_settings.bandwidth = None;
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
        state.filter_widget.set_filter(filter);
    }
    for section in state.sections.iter_mut() {
        match section {
            CustomWidget::Drop(ref mut drop_widget) => {
                if let Some(drop) = &cli.packet_manipulation_settings.drop {
                    drop_widget.set_probability(drop.probability);
                    drop_widget.set_active(true);
                }
            }
            CustomWidget::Delay(ref mut delay_widget) => {
                if let Some(delay) = &cli.packet_manipulation_settings.delay {
                    delay_widget.set_delay(delay.duration);
                    delay_widget.set_active(true);
                }
            }
            CustomWidget::Throttle(ref mut throttle_widget) => {
                if let Some(throttle) = &cli.packet_manipulation_settings.throttle {
                    throttle_widget.set_probability(throttle.probability);
                    throttle_widget.set_throttle_duration(throttle.duration);
                    throttle_widget.drop = throttle.drop;
                    throttle_widget.set_active(true);
                }
            }
            CustomWidget::Reorder(ref mut reorder_widget) => {
                if let Some(reorder) = &cli.packet_manipulation_settings.reorder {
                    reorder_widget.set_probability(reorder.probability);
                    reorder_widget.set_delay_duration(reorder.max_delay);
                    reorder_widget.set_active(true);
                }
            }
            CustomWidget::Tamper(ref mut tamper_widget) => {
                if let Some(tamper) = &cli.packet_manipulation_settings.tamper {
                    tamper_widget.set_probability(tamper.probability);
                    tamper_widget.set_tamper_amount(tamper.amount);
                    if let Some(recalculate_checksums) = tamper.recalculate_checksums {
                        tamper_widget.recalculate_checksums = recalculate_checksums;
                    }
                    tamper_widget.set_active(true);
                }
            }
            CustomWidget::Duplicate(ref mut duplicate_widget) => {
                if let Some(duplicate) = &cli.packet_manipulation_settings.duplicate {
                    duplicate_widget.set_probability(duplicate.probability);
                    duplicate_widget.set_duplicate_count(duplicate.count);
                    duplicate_widget.set_active(true);
                }
            }
            CustomWidget::Bandwidth(ref mut bandwidth_widget) => {
                if let Some(bandwidth) = &cli.packet_manipulation_settings.bandwidth {
                    bandwidth_widget.set_limit(bandwidth.limit);
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

    if let Ok(filter) = &state.filter_widget.filter {
        cli.filter = Some(filter.to_string());
    }

    for section in state.sections.iter_mut() {
        match section {
            CustomWidget::Drop(ref mut drop_widget) => {
                cli.packet_manipulation_settings.drop = if !drop_widget.is_active() {
                    None
                } else {
                    match drop_widget.probability {
                        Ok(probability) => Some(DropOptions { probability }),
                        Err(_) => None,
                    }
                }
            }
            CustomWidget::Delay(ref mut delay_widget) => {
                cli.packet_manipulation_settings.delay = if !delay_widget.is_active() {
                    None
                } else {
                    match delay_widget.delay {
                        Ok(duration) => Some(DelayOptions { duration }),
                        Err(_) => None,
                    }
                }
            }
            CustomWidget::Throttle(ref mut throttle_widget) => {
                cli.packet_manipulation_settings.throttle = if !throttle_widget.is_active() {
                    None
                } else {
                    throttle_widget
                        .probability
                        .as_ref()
                        .ok()
                        .and_then(|probability| {
                            throttle_widget
                                .throttle_duration
                                .as_ref()
                                .ok()
                                .map(|duration| ThrottleOptions {
                                    probability: *probability,
                                    duration: *duration,
                                    drop: throttle_widget.drop,
                                })
                        })
                }
            }
            CustomWidget::Reorder(ref reorder_widget) => {
                cli.packet_manipulation_settings.reorder = if !reorder_widget.is_active() {
                    None
                } else {
                    reorder_widget
                        .probability
                        .as_ref()
                        .ok()
                        .and_then(|probability| {
                            reorder_widget
                                .delay_duration
                                .as_ref()
                                .ok()
                                .map(|max_delay| ReorderOptions {
                                    probability: *probability,
                                    max_delay: *max_delay,
                                })
                        })
                }
            }

            CustomWidget::Tamper(ref tamper_widget) => {
                cli.packet_manipulation_settings.tamper =
                    if !tamper_widget.is_active() {
                        None
                    } else {
                        tamper_widget
                            .probability
                            .as_ref()
                            .ok()
                            .and_then(|probability| {
                                tamper_widget.tamper_amount.as_ref().ok().map(|amount| {
                                    TamperOptions {
                                        probability: *probability,
                                        amount: *amount,
                                        recalculate_checksums: Some(
                                            tamper_widget.recalculate_checksums,
                                        ),
                                    }
                                })
                            })
                    }
            }
            CustomWidget::Duplicate(ref duplicate_widget) => {
                cli.packet_manipulation_settings.duplicate = if !duplicate_widget.is_active() {
                    None
                } else {
                    duplicate_widget
                        .probability
                        .as_ref()
                        .ok()
                        .and_then(|probability| {
                            duplicate_widget.duplicate_count.as_ref().ok().map(|count| {
                                DuplicateOptions {
                                    probability: *probability,
                                    count: *count,
                                }
                            })
                        })
                }
            }
            CustomWidget::Bandwidth(ref bandwidth_widget) => {
                cli.packet_manipulation_settings.bandwidth = if !bandwidth_widget.is_active() {
                    None
                } else {
                    match bandwidth_widget.limit {
                        Ok(limit) => Some(BandwidthOptions { limit }),
                        Err(_) => None,
                    }
                }
            }
        }
    }
}

fn update_tui_state_from_statistics(
    state: &mut TuiState,
    statistics: &Arc<RwLock<PacketProcessingStatistics>>,
) {
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
                CustomWidget::Drop(ref mut drop_widget) => {
                    drop_widget.update_data(&stats.drop_stats);
                }
                CustomWidget::Delay(ref mut delay_widget) => {
                    delay_widget.update_data(&stats.delay_stats);
                }
                CustomWidget::Throttle(ref mut throttle_widget) => {
                    throttle_widget.update_data(&stats.throttle_stats);
                }
                CustomWidget::Reorder(ref mut reorder_widget) => {
                    reorder_widget.update_data(&stats.reorder_stats)
                }
                CustomWidget::Tamper(ref mut tamper_widget) => {
                    tamper_widget.update_data(&stats.tamper_stats)
                }
                CustomWidget::Duplicate(ref mut duplicate_widget) => {
                    duplicate_widget.update_data(&stats.duplicate_stats)
                }
                CustomWidget::Bandwidth(ref mut bandwidth_widget) => {
                    bandwidth_widget.update_data(&stats.bandwidth_stats)
                }
            }
        }
    }
}
