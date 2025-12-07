//! Main application entry point (native).

#[cfg(feature = "native")]
fn main() {
    env_logger::init();
    log::info!("Starting DrafftInk");

    pollster::block_on(drafftink_app::App::run());
}

#[cfg(not(feature = "native"))]
fn main() {
    panic!("Native feature not enabled. Use `cargo run --features native`");
}
