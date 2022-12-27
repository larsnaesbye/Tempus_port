use eframe;
use eframe::{HardwareAcceleration, Theme};
use egui::menu;
use eframe::egui_glow::ShaderVersion;
use hifitime::{Epoch, TimeScale};
use std::io::Error;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Error> {
    let options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
	centered: true,
        decorated: true,
	event_loop_builder: None,
        fullscreen: false,
	fullsize_content: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: None,
        min_window_size: None,
        max_window_size: None,
	mouse_passthrough: false,
        resizable: true,
	shader_version: Some(ShaderVersion::Gl140),
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
		let now = Epoch::from_gregorian_utc_hms(2022, 12, 9, 22, 01, 17);
            ui.horizontal(|ui| {
                ui.label("UTC");
                ui.label(format!("{now}"));
            });
            ui.horizontal(|ui| {
                ui.label("TAI");
                ui.label(format!("{now:x}"));
            });
            ui.horizontal(|ui| {
                ui.label("TT");
                ui.label(format!("{now:X}"));
            });
            ui.horizontal(|ui| {
                ui.label("TDB");
                ui.label(format!("{now:E}"));
            });
            ui.horizontal(|ui| {
                ui.label("ET");
                ui.label(format!("{now:e}"));
            });
            ui.horizontal(|ui| {
                ui.label("UNIX");
                ui.label(format!("{now:p}"));
            });
            ui.horizontal(|ui| {
                ui.label("GPS");
                ui.label(format!("{now:o}"));
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
