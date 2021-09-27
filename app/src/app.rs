use std::{thread, time};
use std::collections::BTreeMap;

use base64;
use eframe::egui::{CtxRef, Rgba, Vec2};
use eframe::egui::{self, FontDefinitions, FontFamily, TextStyle};
use eframe::epi::{Frame, Storage};
use egui::Event;
use md5;
use urlencoding;
use std::borrow::Cow;

lazy_static! {
    static ref FONTS_NAME: String = String::from("JiZiJingDianZhunYuanJianFan");
}

const NAME: &str = "Transformer";

#[derive(Default)]
pub struct Transformer {
    text: String,
    text_base64_encode: String,
    text_base64_decode: String,
    text_url_encode: String,
    text_url_decode: String,
    text_md5: String,
}

impl epi::App for Transformer {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.editor_ui(ui)
        });
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
    fn editor_ui(&mut self, ui: &mut egui::Ui) {
        let Self {
            text,
            text_base64_encode,
            text_base64_decode,
            text_url_encode,
            text_url_decode,
            text_md5
        } = self;

        ui.add(egui::TextEdit::multiline(text).desired_width(f32::INFINITY));
        ui.add(egui::TextEdit::multiline(text_base64_encode).desired_width(f32::INFINITY));
        ui.add(egui::TextEdit::multiline(text_base64_decode).desired_width(f32::INFINITY));
        ui.add(egui::TextEdit::multiline(text_url_encode).desired_width(f32::INFINITY));
        ui.add(egui::TextEdit::multiline(text_url_decode).desired_width(f32::INFINITY));
        ui.add(egui::TextEdit::multiline(text_md5).desired_width(f32::INFINITY));

        for event in &ui.input().events {
            let changed = match event {
                Event::Text|Event::Copy|Event::Key => true,
                _ => false,
            };

            if changed {
                *text_base64_encode = base64::encode(text.clone());
                *text_url_encode = urlencoding::encode(text.clone().as_str()).to_string();
                *text_md5 = format!("{:x}", md5::compute(text.clone()));

                match base64::decode(text.clone()) {
                    Ok(v) => {
                        match String::from_utf8(v) {
                            Ok(str) => {
                                *text_base64_decode = str;
                            }
                            Err(e) => {
                                *text_base64_decode = e.to_string();
                            }
                        }
                    }
                    Err(e) => {
                        *text_base64_decode = e.to_string();
                    }
                }

                match urlencoding::decode(text.clone().as_str()) {
                    Ok(v) => {
                        *text_url_decode = v.to_string()
                    }
                    Err(e) => {
                        *text_url_decode = e.to_string();
                    }
                }
            }
        }
    }

    fn set_fonts(&mut self, egui_ctx: &egui::CtxRef) {
        let mut fonts: FontDefinitions = FontDefinitions::default();

        fonts.font_data.insert(
            FONTS_NAME.to_string(),
            Cow::Borrowed(include_bytes!("../../resource/JiZiJingDianZhunYuanJianFan-Shan(GEETYPE-ZhunYuanGBT-Flash)-2.ttf")),
        );

        fonts.fonts_for_family.get_mut(&FontFamily::Monospace).unwrap().insert(0, FONTS_NAME.to_string());
        fonts.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap().insert(0, FONTS_NAME.to_string());

        fonts.family_and_size = BTreeMap::new();
        fonts.family_and_size.insert(TextStyle::Small, (FontFamily::Proportional, 16.0));
        fonts.family_and_size.insert(TextStyle::Body, (FontFamily::Proportional, 24.0));
        fonts.family_and_size.insert(TextStyle::Button, (FontFamily::Proportional, 24.0));
        fonts.family_and_size.insert(TextStyle::Heading, (FontFamily::Proportional, 28.0));
        fonts.family_and_size.insert(TextStyle::Monospace, (FontFamily::Monospace, 26.0));

        egui_ctx.set_fonts(fonts);
    }
}