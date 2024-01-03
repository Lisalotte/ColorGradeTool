use egui::Vec2;

pub mod app;
pub mod colorgrade;

fn main() {
    println!("Hello, world!");

    // Settings for the egui viewport

    let windowsize = Vec2::new(800.0, 680.0);

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(windowsize),
        ..Default::default()
    };

    eframe::run_native(
        "ColorGrade Tool",
        native_options,
        Box::new(|cc| Box::new(app::ColorGradeApp::new(cc)))
    ).unwrap();
}