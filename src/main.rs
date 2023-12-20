use egui::{ViewportBuilder, Vec2};

pub mod app;
pub mod colorgrade;

fn main() {
    println!("Hello, world!");

    // Settings for the egui viewport

    let windowsize = Vec2::new(800.0, 600.0);

    let viewport_builder = egui::viewport::ViewportBuilder {
        title: Option::Some(String::from("Color Grade Tool")),
        app_id: Option::Some(String::from("colorgradetool")),
        position: Option::None,
        inner_size: Option::Some(windowsize),
        min_inner_size: Option::None,
        max_inner_size: Option::None,
        fullscreen: Option::Some(false),
        maximized: Option::Some(false),
        resizable: Option::Some(true),
        transparent: Option::Some(false),
        decorations: Option::Some(true),
        icon: Option::None,
        active: Option::Some(true),
        visible: Option::Some(true),
        drag_and_drop: Option::Some(true),
        fullsize_content_view: Option::Some(true),
        title_shown: Option::Some(true),
        titlebar_buttons_shown: Option::Some(true),
        titlebar_shown: Option::Some(true),
        close_button: Option::Some(true),
        minimize_button: Option::Some(true),
        maximize_button: Option::Some(true),
        window_level: Option::Some(egui::viewport::WindowLevel::Normal),
        mouse_passthrough: Option::None
    };

    let native_options = eframe::NativeOptions { 
        viewport: viewport_builder, 
        vsync: true, 
        multisampling: 0, 
        depth_buffer: 0, 
        stencil_buffer: 0, 
        hardware_acceleration: eframe::HardwareAcceleration::Preferred, 
        renderer: eframe::Renderer::Glow, 
        follow_system_theme: true, 
        default_theme: eframe::Theme::Dark, 
        run_and_return: true, 
        event_loop_builder: Option::None, 
        window_builder: Option::None, 
        shader_version: Option::None, 
        centered: true, 
        persist_window: true 
    };

    eframe::run_native(
        "ColorGrade Tool",
        native_options,
        Box::new(|cc| Box::new(app::ColorGradeApp::new(cc)))
    ).unwrap();
}