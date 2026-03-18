use core::{fmt::Write, time::Duration};
use std::time::SystemTime;

use eframe::{
    App, NativeOptions,
    egui::{self, Align, Color32, Layout, ProgressBar, RichText, Ui, Vec2, ViewportBuilder},
};

use crate::consts::{DEFAULT_DELAY, DEFAULT_LENGTH, TEXT_LOOK_FAR, TEXT_SETTINGS, TEXT_UNTIL_NEXT};

pub mod consts;

pub struct TwentyCubedApp {
    /// The current countdown state.
    countdown_state: CountdownState,
    /// Whether or not the settings menu is currently open.
    settings_open: bool,
    /// The delay between breaks.
    delay: Duration,
    /// The length of the break.
    length: Duration,
}

impl TwentyCubedApp {
    #[must_use]
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            countdown_state: CountdownState::default(),
            settings_open: false,
            delay: DEFAULT_DELAY,
            length: DEFAULT_LENGTH,
        }
    }

    fn ui(&mut self, ui: &mut Ui) {
        if self.settings_open {
            self.ui_settings(ui);
            return;
        }

        match self.countdown_state {
            CountdownState::WaitingForDelay { wait_start } => {
                self.settings_open = Self::ui_waiting(ui, TEXT_UNTIL_NEXT, wait_start, self.delay);
            }
            CountdownState::PreBreak => {
                let confirmed = Self::ui_confirm(ui);
                if confirmed {
                    self.countdown_state = CountdownState::WaitingForLength {
                        wait_start: SystemTime::now(),
                    }
                }
            }
            CountdownState::WaitingForLength { wait_start } => {
                self.settings_open = Self::ui_waiting(ui, TEXT_LOOK_FAR, wait_start, self.length);
            }
        }
    }

    fn ui_settings(&mut self, ui: &mut Ui) {
        let elapsed_and_total = self
            .countdown_state
            .as_elapsed_and_total(self.delay, self.length);
        let remaining =
            elapsed_and_total.map_or(Duration::ZERO, |(elapsed, total)| total - elapsed);
        let total = elapsed_and_total.map_or(Duration::ZERO, |(_, t)| t);

        ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
            duration_progress(ui, remaining, total);

            let settings_clicked = ui.selectable_label(true, TEXT_SETTINGS).clicked();
            if settings_clicked {
                self.settings_open = false;
            }
        });
    }

    /// Returns whether or not the user confirmed starting the break.
    #[must_use]
    fn ui_confirm(ui: &mut Ui) -> bool {
        todo!();
    }

    /// Returns whether or not the user toggled on the settings menu.
    #[must_use]
    fn ui_waiting(ui: &mut Ui, text: &str, wait_start: SystemTime, wait_time: Duration) -> bool {
        let elapsed = wait_start.elapsed().unwrap_or_default();
        let remaining = wait_time.saturating_sub(elapsed);

        ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
            duration_progress(ui, remaining, wait_time);
            let settings_clicked = ui.selectable_label(false, TEXT_SETTINGS).clicked();

            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                ui.label(text);
                ui.centered_and_justified(|ui| {
                    main_duration(ui, remaining);
                });
            });
            settings_clicked
        })
        .inner
    }
}

impl App for TwentyCubedApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.countdown_state.update(self.delay, self.length);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });

        ctx.request_repaint_after(Duration::from_millis(250));
    }
}

/// The current thing the countdown is waiting for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum CountdownState {
    /// Waiting for the delay between two breaks.
    WaitingForDelay { wait_start: SystemTime },
    /// Waiting for user confirmation to start a break.
    PreBreak,
    /// Waiting for the length of the break.
    WaitingForLength { wait_start: SystemTime },
}

impl CountdownState {
    fn update(&mut self, delay: Duration, length: Duration) {
        match self {
            CountdownState::WaitingForDelay { wait_start } => {
                let wait_end = *wait_start + delay;
                if wait_end.elapsed().is_ok() {
                    *self = CountdownState::PreBreak;
                }
            }
            CountdownState::PreBreak => (),
            CountdownState::WaitingForLength { wait_start } => {
                let wait_end = *wait_start + length;
                if wait_end.elapsed().is_ok() {
                    *self = CountdownState::WaitingForDelay {
                        wait_start: wait_end,
                    };
                }
            }
        }
    }

    #[must_use]
    fn as_elapsed_and_total(
        self,
        delay: Duration,
        length: Duration,
    ) -> Option<(Duration, Duration)> {
        match self {
            CountdownState::PreBreak => None,
            CountdownState::WaitingForLength { wait_start } => {
                Some((wait_start.elapsed().unwrap_or_default(), length))
            }
            CountdownState::WaitingForDelay { wait_start } => {
                Some((wait_start.elapsed().unwrap_or_default(), delay))
            }
        }
    }
}

impl Default for CountdownState {
    fn default() -> Self {
        Self::WaitingForDelay {
            wait_start: SystemTime::now(),
        }
    }
}

fn format_duration(duration: Duration) -> String {
    let h = duration.div_duration_f64(Duration::from_hours(1)).floor();
    let rem = duration
        .checked_sub(Duration::from_hours(1).mul_f64(h))
        .unwrap();
    let m = rem.div_duration_f64(Duration::from_mins(1)).floor();
    let rem = rem.checked_sub(Duration::from_mins(1).mul_f64(m)).unwrap();
    let s = rem.as_secs_f64().floor();

    let mut string = String::with_capacity(2 + 1 + 2 + 1 + 2);

    if h > 0.0 {
        write!(string, "{h:02}:").expect("formatting and writing should work");
    }

    write!(string, "{m:02}:{s:02}").expect("formatting and writing should work");

    string
}

fn main_duration(ui: &mut Ui, duration: Duration) {
    let string = format_duration(duration);
    #[expect(clippy::cast_precision_loss)]
    let font_size = ui.available_width() / string.len() as f32;
    let font_size = font_size.min(ui.available_height() / 2.0);

    ui.label(RichText::new(string).color(Color32::WHITE).size(font_size));
}

fn duration_progress(ui: &mut Ui, duration: Duration, total: Duration) {
    let duration = Duration::from_secs(duration.as_secs());
    ui.add(
        ProgressBar::new(duration.div_duration_f32(total))
            .desired_width(ui.available_width())
            .desired_height(4.0)
            .fill(Color32::WHITE)
            .corner_radius(0),
    );
}

#[must_use]
pub fn create_native_options() -> NativeOptions {
    NativeOptions {
        viewport: ViewportBuilder {
            min_inner_size: Some(Vec2::splat(48.0)),
            ..Default::default()
        },
        ..Default::default()
    }
}
