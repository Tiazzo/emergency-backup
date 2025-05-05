use eframe::egui::{self, Button};
use egui::viewport::ViewportBuilder;
use serde::{Deserialize, Serialize};

use image::open;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AppConfig {
    pub source_dir: String,     
    pub file_extension: String, 
}

impl AppConfig {
    pub fn load() -> Self {
        if Path::new("config.json").exists() {
            let file = File::open("config.json").expect("Unable to open configuration file");
            serde_json::from_reader(file).expect("Configuration file parsing error")
        } else {
            Self::default() // Return default config if no config file exists
        }
    }

    /// Saves the current configuration to `config.json`
    fn save(&self) {
        let file = File::create("config.json").expect("Unable to create configuration file");
        serde_json::to_writer_pretty(file, self).expect("Error saving configuration");
    }
}

pub struct MyApp {
    config: AppConfig,
    selected_source: String,
    selected_extension: String,
}

impl MyApp {
    /// Initializes a new instance of the app, loading the configuration from the file
    pub fn new() -> Self {
        let config = AppConfig::load();
        Self {
            config: config.clone(),
            selected_source: config.source_dir.clone(),
            selected_extension: config.file_extension.clone(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0); 
                ui.heading("Select backup source:"); 
                ui.add_space(10.0);

                // Button to pick the source folder
                if ui
                    .add(
                        Button::new(&self.selected_source)
                            .min_size(egui::vec2(40.0, 30.0))
                            .truncate(),
                    )
                    .clicked()
                {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.selected_source = path.display().to_string(); // Update selected source path
                    }
                }

                ui.add_space(30.0);
                ui.heading("File extension to include (optional):"); // Heading for file extension input
                ui.add_space(10.0);

                // Input field for file extension
                ui.text_edit_singleline(&mut self.selected_extension);
                ui.add_space(30.0);

                // Confirm button to save the configuration
                if ui
                    .add(Button::new("Confirm").min_size(egui::vec2(80.0, 40.0)))
                    .clicked()
                {
                    // Save the selected configuration
                    self.config.source_dir = self.selected_source.clone();
                    self.config.file_extension = self.selected_extension.clone();
                    self.config.save();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }
}

pub fn start_gui() {
    let img_path = if cfg!(debug_assertions) {
        std::path::Path::new("src/images/backup_icon.png").to_path_buf()     // Load the backup icon image
    } else {
        let exe_path = std::env::current_exe().unwrap();
        exe_path.parent().unwrap().join("images/backup_icon.png")
    };

    let img = open(img_path).expect("Unable to load image");
    let icon_data = img.to_rgba8().into_raw();

    // Configure the window options
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("Backup Tool") 
            .with_icon(Arc::new(egui::IconData {
                rgba: icon_data, 
                width: img.width() as u32,
                height: img.height() as u32,
            }))
            .with_resizable(false) 
            .with_maximize_button(false) 
            .with_close_button(false) // Disable close button
            .with_max_inner_size(egui::Vec2::new(500.0, 300.0)), 
        ..Default::default()
    };

    // Start the native application
    eframe::run_native(
        "Backup Tool",                              
        options,                                    
        Box::new(|_cc| Ok(Box::new(MyApp::new()))), // Initialize the app
    )
    .expect("Error starting the application");
}
