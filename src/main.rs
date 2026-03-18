use w2_20p3::{TwentyCubedApp, consts::APP_NAME, create_native_options};

fn main() -> eframe::Result {
    let native_options = create_native_options();
    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(TwentyCubedApp::new(cc)))),
    )
}
