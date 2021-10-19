use std::borrow::Cow;
use std::collections::BTreeMap;

use eframe::egui::{self, FontDefinitions, FontFamily, TextStyle};
use eframe::epi::{Frame, Storage};
use egui::Event;

lazy_static! {
    static ref FONTS_NAME: String = String::from("JiZiJingDianZhunYuanJianFan");
}

const NAME: &str = "Transformer";

#[derive(Default)]
pub struct Transformer {
    text: String,
}

impl epi::App for Transformer {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| self.ui_update(ui));
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut Frame<'_>,
        _storage: Option<&dyn Storage>,
    ) {
        self.set_fonts(_ctx);
    }

    fn name(&self) -> &str {
        return NAME;
    }
}

impl Transformer {
    fn ui_update(&mut self, ui: &mut egui::Ui) {
        let Self { text } = self;

        ui.add(egui::TextEdit::multiline(text).desired_width(f32::INFINITY));

        for event in &ui.input().events {
            match event {
                Event::Key { .. } | Event::Text(..) | Event::Copy => (),
                _ => continue,
            };
        }
    }

    fn set_fonts(&mut self, egui_ctx: &egui::CtxRef) {
        let mut fonts: FontDefinitions = FontDefinitions::default();

        fonts.font_data.insert(
            FONTS_NAME.to_string(),
            Cow::Borrowed(include_bytes!(
                "../../resource/JiZiJingDianZhunYuanJianFan-Shan(GEETYPE-ZhunYuanGBT-Flash)-2.ttf"
            )),
        );

        fonts
            .fonts_for_family
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, FONTS_NAME.to_string());
        fonts
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, FONTS_NAME.to_string());

        fonts.family_and_size = BTreeMap::new();
        fonts
            .family_and_size
            .insert(TextStyle::Small, (FontFamily::Proportional, 16.0));
        fonts
            .family_and_size
            .insert(TextStyle::Body, (FontFamily::Proportional, 24.0));
        fonts
            .family_and_size
            .insert(TextStyle::Button, (FontFamily::Proportional, 24.0));
        fonts
            .family_and_size
            .insert(TextStyle::Heading, (FontFamily::Proportional, 28.0));
        fonts
            .family_and_size
            .insert(TextStyle::Monospace, (FontFamily::Monospace, 26.0));

        egui_ctx.set_fonts(fonts);
    }
}
