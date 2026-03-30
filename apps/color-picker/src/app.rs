use eframe::egui::{CentralPanel, Context};
use eframe::Frame;

use crate::preview::update_preview;
use crate::slider::update_slider;
use crate::title::update_title;

#[derive(Default)]
pub struct ColorPickerApp{
}

impl eframe::App for ColorPickerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            update_title(ui);
            update_preview(ui);
            update_slider(ui);
        });
    }
}


