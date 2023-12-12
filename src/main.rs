use eframe::egui;
mod app;
fn main() {
    println!("Hello, world!");

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "ColorGrade Tool",
        native_options,
        Box::new(|cc| Box::new(app::ColorGradeApp::new(cc)))
    );
}