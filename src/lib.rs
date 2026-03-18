use core::{fmt::Write, time::Duration};
use std::time::SystemTime;

use eframe::{
    App, NativeOptions,
    egui::{self, Color32, ProgressBar, RichText, Ui, Vec2, ViewportBuilder},
};

use crate::consts::{DEFAULT_DELAY, DEFAULT_LENGTH, TEXT_UNTIL_NEXT};

pub mod consts;

pub struct TwentyCubedApp {
    last_trigger: SystemTime,
    delay: Duration,
    length: Duration,
}

impl TwentyCubedApp {
    #[must_use]
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            last_trigger: SystemTime::now(),
            delay: DEFAULT_DELAY,
            length: DEFAULT_LENGTH,
        }
    }
}

impl App for TwentyCubedApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let next_trigger = self.last_trigger + self.delay;
            let duration = next_trigger
                .duration_since(SystemTime::now())
                .unwrap_or_default();
            main_duration(duration, ui);
            duration_progress(duration, self.delay, ui);
        });

        ctx.request_repaint_after(Duration::from_millis(250));
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

fn main_duration(duration: Duration, ui: &mut Ui) {
    ui.centered_and_justified(|ui| {
        let string = format_duration(duration);
        #[expect(clippy::cast_precision_loss)]
        let font_size = ui.available_width() / string.len() as f32;
        let font_size = font_size.min(ui.available_height() / 2.0);

        ui.label(RichText::new(string).color(Color32::WHITE).size(font_size));
    });
}

fn duration_progress(duration: Duration, total: Duration, ui: &mut Ui) {
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
