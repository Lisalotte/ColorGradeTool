use egui::Context;
use crate::app::configmanager;
use crate::app::presetmanager;
use super::ColorGradeApp;

pub fn show_presetname_viewport(app: &mut ColorGradeApp, ctx: &Context) {
    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("presetname_viewport"),
        egui::ViewportBuilder::default()
            .with_title("Preset Name")
            .with_inner_size([300.0, 200.0]),
        |ctx, class| {
            assert!(
                class == egui::ViewportClass::Immediate,
                "This egui backend doesn't support multiple viewports"
            );

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Save preset as:");
                ui.text_edit_singleline(&mut app.preset_name);
                if ui.button("Save").clicked() {
                    presetmanager::save_preset(&app.color_grade, &app.preset_name);
                    app.show_presetname_viewport = false;
                }
            });

            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent viewport that we should not show next frame:
                app.show_presetname_viewport = false;
            }
        },
    );
}

pub fn show_path_viewport(app: &mut ColorGradeApp, ctx: &Context) {    
    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("objectpath_viewport"),
        egui::ViewportBuilder::default()
            .with_title("Object Path")
            .with_inner_size([600.0, 200.0]),
        |ctx, class| {
            assert!(
                class == egui::ViewportClass::Immediate,
                "This egui backend doesn't support multiple viewports"
            );

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Object Path:");
                ui.text_edit_singleline(&mut app.object_path);
                if ui.button("Save").clicked() {
                    app.show_path_viewport = false;
                }
            });

            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent viewport that we should not show next frame:
                app.show_path_viewport = false;
            }
        },
    );
}

pub fn show_ip_viewport(app: &mut ColorGradeApp, ctx: &Context) {    
    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("ip_viewport"),
        egui::ViewportBuilder::default()
            .with_title("IP Address")
            .with_inner_size([600.0, 200.0]),
        |ctx, class| {
            assert!(
                class == egui::ViewportClass::Immediate,
                "This egui backend doesn't support multiple viewports"
            );

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("IP Address:");
                ui.text_edit_singleline(&mut app.ip_address);
                if ui.button("Save").clicked() {
                    app.show_ip_viewport = false;
                }
            });

            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent viewport that we should not show next frame:
                app.show_ip_viewport = false;
            }
        },
    );
}

pub fn show_config_button_viewport(app: &mut ColorGradeApp, ctx: &Context) {
     
    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("configure_viewport"),
        egui::ViewportBuilder::default()
            .with_title("Configure Button")
            .with_inner_size([600.0, 400.0]),
        |ctx, class| {
            assert!(
                class == egui::ViewportClass::Immediate,
                "This egui backend doesn't support multiple viewports"
            );

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Project:");
                    ui.text_edit_singleline(&mut app.button_config.project_name); 
                });
                ui.horizontal(|ui| {
                    ui.label("Preset:");
                    ui.text_edit_singleline(&mut app.button_config.preset_name);
                });
                ui.horizontal(|ui| {
                    ui.label("BP Object Path:");
                    ui.text_edit_singleline(&mut app.button_config.object_path);
                });
                ui.horizontal(|ui| {
                    ui.label("IP Address:");
                    ui.text_edit_singleline(&mut app.button_config.ip_address);
                });

                if ui.button("Save").clicked() {
                    configmanager::save_config(
                        "config/buttons",
                        &format!("button{}.json", &app.button_config.button_nr), 
                        &app.button_config.object_path, 
                        &app.button_config.preset_name, 
                        &app.button_config.ip_address, 
                        &app.button_config.project_name);
                    app.show_config_viewport = false;
                }
            });

            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent viewport that we should not show next frame:
                app.show_config_viewport = false;
            }
        },
    );
}

pub fn show_config_viewport(app: &mut ColorGradeApp, ctx: &Context) {
    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("configname_viewport"),
        egui::ViewportBuilder::default()
            .with_title("Config Name")
            .with_inner_size([300.0, 200.0]),
        |ctx, class| {
            assert!(
                class == egui::ViewportClass::Immediate,
                "This egui backend doesn't support multiple viewports"
            );

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Save config as:");
                ui.text_edit_singleline(&mut app.config_name);
                if ui.button("Save").clicked() {
                    configmanager::save_config("config/saved", &format!("{}.json", app.config_name), &app.object_path, &app.preset_name, &app.ip_address, &app.project_name);
                    app.show_config_viewport = false;
                }
            });

            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent viewport that we should not show next frame:
                app.show_config_viewport = false;
            }
        },
    );
}