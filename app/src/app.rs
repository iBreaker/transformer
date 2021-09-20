use eframe::egui::{CtxRef, Vec2, Rgba};
use eframe::epi::{Frame, Storage};

const  NAME:&str = "Transformer";

#[derive(Default)]
pub struct Transformer {
    text: String,
}

impl epi::App for Transformer{
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut Frame<'_>){

    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut Frame<'_>,
        _storage: Option<&dyn Storage>,
    ) {
    }

    fn name(&self) -> &str{
        return NAME
    }
}