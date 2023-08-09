use eframe::egui;
use egui::{Visuals, Style};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Star Trek Picker",
        options,
        Box::new(|cc| {
            let style = Style {
                visuals: Visuals::dark(),
                ..Style::default()
            };
            cc.egui_ctx.set_style(style);
            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    button1: bool,
    button2: bool,
    button3: bool,
    episode: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            button1: false,
            button2: false,
            button3: false,
            episode: "No Episode Picked".to_string()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Jimmy's Star Trek Picker");
            ui.checkbox(&mut self.button1, "Button 1");
            ui.checkbox(&mut self.button2, "Button 2");
            ui.checkbox(&mut self.button3, "Button 3");
            ui.horizontal(|ui| {
                if ui.button("Generate Episode").clicked() {
                    self.episode = format!("Episode is: {}",5);
                }
                ui.label(&self.episode);
            });
        });
    }
}
