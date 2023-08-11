#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use egui::{Visuals, Style};
use rand_gen::*;

pub mod rand_gen;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
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
            cc.egui_ctx.set_pixels_per_point(1.5);
            Box::<Options>::default()
        }),
    )
}

impl eframe::App for Options {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Jimmy's Star Trek Picker");
            ui.hyperlink_to("My GitHub", "https://github.com/LordGoatius");
            ui.columns(3, |columns| {
                columns[0].label("TV Shows");
                columns[0].checkbox(&mut self.tos, "The Original Series");
                columns[0].checkbox(&mut self.tas, "The Animated Series");
                columns[0].checkbox(&mut self.tng, "The Next Generation");
                columns[0].checkbox(&mut self.ds9, "Deep Space Nine");
                columns[0].checkbox(&mut self.voy, "Voyager");
                columns[0].checkbox(&mut self.ent, "Enterprise");
                columns[0].checkbox(&mut self.dis, "Discovery");
                columns[0].checkbox(&mut self.pic, "Picard");
                columns[0].checkbox(&mut self.ld, "Lower Decks");
                columns[0].checkbox(&mut self.snw, "Strange New Worlds");
                columns[0].checkbox(&mut self.prodigy, "Prodigy");
                
                

                columns[1].horizontal(|horiz_ui| {
                    horiz_ui.label("Movies:");
                    horiz_ui.checkbox(&mut self.show_movies, "Show Movies");
                });
                if self.show_movies {
                    columns[1].checkbox(&mut self.tmp, "The Motion Picture");
                    columns[1].checkbox(&mut self.twok, "The Wrath of Khan");
                    columns[1].checkbox(&mut self.tsfs, "The Search for Spock");
                    columns[1].checkbox(&mut self.tvh, "The Voyage Home");
                    columns[1].checkbox(&mut self.tff, "The Final Frontier");
                    columns[1].checkbox(&mut self.tuc, "The Undiscovered Country");
                
                    columns[1].checkbox(&mut self.stg, "Generations");
                    columns[1].checkbox(&mut self.stfc, "First Contact");
                    columns[1].checkbox(&mut self.sti, "Insurrection");
                    columns[1].checkbox(&mut self.stn, "Nemesis");
                
                    columns[1].checkbox(&mut self.stk, "Star Trek (2009)");
                    columns[1].checkbox(&mut self.stid, "Star Trek: Into Darkness");
                    columns[1].checkbox(&mut self.stb, "Star Trek: Beyond");
                }

                columns[2].horizontal(|horiz_ui| {
                    horiz_ui.label("Presets");
                    if horiz_ui.button("Deselect All").clicked() { self.deselect_all(); }
                });

                if columns[2].button("All Shows").clicked() { self.all_shows(); }
                if columns[2].button("All Movies").clicked() { self.all_movies(); }
                if columns[2].button("Big 3 Shows").clicked() { self.big_3_shows(); }
                if columns[2].button("On a Ship").clicked() { self.shows_on_ship(); }
                if columns[2].button("Classic Shows").clicked() { self.classic(); }
                if columns[2].button("New Shows").clicked() { self.new(); }
                if columns[2].button("Classic Movies").clicked() { self.classic_movies(); }
                if columns[2].button("TNG Movies").clicked() { self.tng_movies(); }
                if columns[2].button("Kelvin Movies").clicked() { self.kelvin_movies(); }
                if columns[2].button("A Fun Time :)").clicked() { self.a_fun_time(); }
                if columns[2].button("Jimmy's Choice").clicked() { self.jimmys_recommended(); }
            });
            ui.horizontal(|horiz_ui| {
                if horiz_ui.button("Generate Episode").clicked() {
                    let seasons = Seasons::default();
                    select_from_options(self, &seasons);
                }
                horiz_ui.label(&self.episode);
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
