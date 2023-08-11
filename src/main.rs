#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use egui::{Visuals, Style};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        icon_data : Some(load_icon()),
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
            ui.hyperlink_to("My GitHub", "https://github.com/LordGoatius");
            ui.columns(2, |columns| {
                columns[0].checkbox(&mut self.button1, "Button 1");
                columns[0].checkbox(&mut self.button2, "Button 2");
                columns[0].checkbox(&mut self.button3, "Button 3");
                if self.button3 {
                    columns[0].checkbox(&mut self.button3, "Congrats, you found the hidden button");
                }
                columns[0].horizontal(|horiz_ui| {
                    if horiz_ui.button("Generate Episode").clicked() {
                        self.episode = format!("Episode is: {}",5);
                    }
                    horiz_ui.label(&self.episode);
                });

                columns[1].label("Label yay");
                if columns[1].button("BUdton").clicked() {
                    columns[1].label("Budton");
                }
            });
        });
    }
}

fn load_icon() -> eframe::IconData {
	let (icon_rgba, icon_width, icon_height) = {
		let icon = include_bytes!("../startrek.png");
		let image = image::load_from_memory(icon)
			.expect("Failed to open icon path")
			.into_rgba8();
		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		(rgba, width, height)
	};
	
	eframe::IconData {
		rgba: icon_rgba,
		width: icon_width,
		height: icon_height,
	}
}
