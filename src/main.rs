#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
mod lib;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Translater",
        options,
        Box::new(|cc| Box::new(Translater::new(cc))),
    );
}



struct Translater {
    from: String,
    to: String,
}

impl Translater {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        lib::setup_custom_fonts(&cc.egui_ctx);
        Self {
            from: "word".to_owned(),
            to: "".to_owned()
        }
    }
}

impl eframe::App for Translater {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let resp = ui.text_edit_multiline(&mut self.from);
            if resp.changed(){
                self.to.clear();
                self.to.push_str(&self.from);
                self.to.push_str("\n translated");
            }
            ui.text_edit_multiline(&mut self.to);
        });
    }
}