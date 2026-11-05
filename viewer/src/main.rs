#![windows_subsystem = "windows"]

mod app;
mod data;
mod style;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_title("SortPort").with_inner_size([1100.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "SortPort",
        options,
        Box::new(|cc| {
            style::apply(&cc.egui_ctx);
            Ok(Box::new(app::Viewer::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../output"))))
        }),
    )
}