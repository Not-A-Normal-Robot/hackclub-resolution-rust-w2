use core::{fmt::Write, time::Duration};
use std::time::SystemTime;

use eframe::{
    App, NativeOptions,
    egui::{self, Color32, ProgressBar, RichText, Ui, Vec2, ViewportBuilder},
};

use crate::consts::{DEFAULT_DELAY, DEFAULT_LENGTH, TEXT_LOOK_FAR, TEXT_UNTIL_NEXT};

pub mod consts;

pub struct TwentyCubedApp {
    /// The current countdown state.
    countdown_state: CountdownState,
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
            delay: DEFAULT_DELAY,
            length: DEFAULT_LENGTH,
        }
    }
}

impl App for TwentyCubedApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.countdown_state.update(self.delay, self.length);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.countdown_state.ui(ui, self.delay, self.length);
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

    fn ui(&self, ui: &mut Ui, delay: Duration, length: Duration) {
        match self {
            CountdownState::WaitingForDelay { wait_start } => {
                Self::ui_waiting(ui, TEXT_UNTIL_NEXT, *wait_start, delay);
            }
            CountdownState::PreBreak => {
                todo!();
            }
            CountdownState::WaitingForLength { wait_start } => {
                Self::ui_waiting(ui, TEXT_LOOK_FAR, *wait_start, length);
            }
        }
    }

    fn ui_waiting(ui: &mut Ui, text: &str, wait_start: SystemTime, wait_time: Duration) {
        let elapsed = wait_start.elapsed().unwrap_or_default();
        let remaining = wait_time.saturating_sub(elapsed);

        ui.label(text);
        main_duration(ui, remaining);
        duration_progress(ui, remaining, wait_time);
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
    ui.centered_and_justified(|ui| {
        let string = format_duration(duration);
        #[expect(clippy::cast_precision_loss)]
        let font_size = ui.available_width() / string.len() as f32;
        let font_size = font_size.min(ui.available_height() / 2.0);

        ui.label(RichText::new(string).color(Color32::WHITE).size(font_size));
    });
}

fn duration_progress(ui: &mut Ui, duration: Duration, total: Duration) {
    let duration = Duration::from_secs(duration.as_secs());
    ui.add(
        ProgressBar::new(duration.div_duration_f32(total))
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
