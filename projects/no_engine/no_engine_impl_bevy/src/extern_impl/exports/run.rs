use bevy::{app::App, log::LogPlugin};

/// This call initializes the logging plugin which allows the `info!` macro to
/// work.
#[unsafe(no_mangle)]
pub extern "C" fn run() {
    let mut app = App::new();
    app.add_plugins(LogPlugin::default());
}
