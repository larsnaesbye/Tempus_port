use eframe;
use hifitime::Epoch;
use std::io::Error;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        &("Tempus ".to_owned() + VERSION.unwrap_or("unknown")),
        options,
        Box::new(|_cc| Box::new(TempusApp::default())),
    );

    return Ok(());
}

struct TempusApp {
    name: String,
    age: u32,
}

impl eframe::App for TempusApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });
            ui.menu_button("Time", |ui| {
                if ui.button("About").clicked() {
                    // …
                }
            });
            ui.menu_button("Location", |ui| {
                if ui.button("About").clicked() {
                    // …
                }
            });
            ui.menu_button("Help", |ui| {
                if ui.button("About").clicked() {
                    // …
                }
            });

            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=132).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("{}", Epoch::now().unwrap()));
        });
    }
}
impl Default for TempusApp {
    fn default() -> Self {
        Self {
            name: "Lars".to_owned(),
            age: 44,
        }
    }
}
