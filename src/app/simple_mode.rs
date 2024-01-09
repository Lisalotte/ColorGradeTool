use crate::app::configmanager;
use egui::Context;

pub fn buttons(ui: &mut egui::Ui, _ctx: &Context, _clicked: &mut bool, _button_clicked: &mut i32) {

    // For all files in the config folder
    if let Ok(_current_dir) = std::env::current_dir() {
        let config_folder = "config/buttons";

        let paths = std::fs::read_dir(config_folder).unwrap();

        let mut counter = 0;
        let col_max = 2;
        let mut col = 1;

        egui::Grid::new("new_grid").show(ui, |ui| {
            for path in paths {
                // Create a button, with a maximum of 10 buttons
                let mut project_name = String::from("");

                let actualpath = path.unwrap().path();

                configmanager::get_projectname(&actualpath, &mut project_name);
            
                let text = format!("{} - {}", project_name, configmanager::get_presetname(&actualpath));

                ui.horizontal(|ui| {
                    ui.label(text)
                });

                ui.label(" ");

                if col == col_max {
                    ui.end_row();
                    col = 1;
                } else {
                    col += 1;
                }

                if counter > 9 {
                    break;
                } else {
                    counter += 1;
                }
            }
        });
    }

    // Load the preset connected to the first button (which is always default.json - for now)
}