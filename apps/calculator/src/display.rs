use eframe::egui;
use eframe::egui::{Color32, Ui};

use crate::{MARGIN, WINDOW_WIDTH};

pub const DISPLAY_HEIHGT: f32 = 25.0;
pub const DISLAY_WIDTH: f32 = WINDOW_WIDTH - 2.0 * MARGIN;
pub const FONT_SIZE: f32 = 12.0;

#[derive(Default)]
pub struct Display;

impl Display {
    pub fn add_main_display(&self, ui: &mut Ui, text: &mut String) {
        self.add_display(ui, text, Color32::BLACK, Color32::WHITE);
    }

    pub fn add_message_display(&self, ui: &mut Ui, text: &mut String) {
        self.add_display(ui, text, Color32::RED, Color32::LIGHT_GRAY);
    }

    fn add_display(
        &self,
        ui: &mut Ui,
        text: &mut String,
        text_color: Color32,
        background: Color32,
    ) {
        ui.add_sized(
            [DISLAY_WIDTH, DISPLAY_HEIHGT],
            egui::TextEdit::singleline(text)
                .background_color(background)
                .text_color(text_color)
                .font(egui::FontId::proportional(FONT_SIZE))
                .horizontal_align(egui::Align::Max),
        );
    }
}
