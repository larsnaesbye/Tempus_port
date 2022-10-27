use eframe;
use eframe::{HardwareAcceleration, Theme};
use egui::menu;
use hifitime::Epoch;
use std::io::Error;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Error> {
    let options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: None,
        min_window_size: None,
        max_window_size: None,
        resizable: true,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: HardwareAcceleration::Required,
        renderer: Default::default(),
        follow_system_theme: true,
        default_theme: Theme::Dark,
        run_and_return: false,
    };
    eframe::run_native(
        &("Tempus ".to_owned() + VERSION.unwrap_or("unknown version")),
        options,
        Box::new(|_cc| Box::new(TempusApp::default())),
    );

    return Ok(());
}

struct TempusApp {}

impl eframe::App for TempusApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About...").clicked() {
                        // …
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Universal Time, Coordinated");
                ui.label(format!("{}", Epoch::now().unwrap()));
            });
            ui.horizontal(|ui| {
                ui.label("Local Mean Time");
                ui.label(format!("{}", Epoch::now().unwrap()));
            });
            ui.horizontal(|ui| {
                ui.label("Julian Day");
                ui.label(format!("{}", Epoch::now().unwrap()));
            });
            ui.horizontal(|ui| {
                ui.label("Julian Day, modified");
                ui.label(format!("{}", Epoch::now().unwrap()));
            });
            ui.horizontal(|ui| {
                ui.label("Local Sidereal Time");
                ui.label(format!("{}", Epoch::now().unwrap()));
            });
            ui.horizontal(|ui| {
                ui.label("Greenwich Sidereal Time");
                ui.label(format!("{}", Epoch::now().unwrap()));
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.label("");
        });
    }
}
impl Default for TempusApp {
    fn default() -> Self {
        Self {}
    }
}
