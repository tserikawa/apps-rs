use eframe::egui::{self, Color32};

use crate::{calculator::Calculator, keypad::{BUTTON_HEIGHT, BUTTON_WIDTH, Keypad}};

mod calculator;
mod keypad;
mod decoder;

pub const APP_NAME: &str = "Calculator";
pub const MARGIN: f32 = 6.0;
pub const WINDOW_WIDTH: f32 = BUTTON_WIDTH * 4.0 + MARGIN * 5.0;
pub const WINDOW_HEIGHT: f32 = BUTTON_HEIGHT * 4.0 + MARGIN * 7.0 + 2.0 * MONITOR_HEIHGT;
pub const MONITOR_HEIHGT: f32 = 25.0;
pub const MONITOIR_WIDTH: f32 = WINDOW_WIDTH - 2.0 * MARGIN;
pub const FONT_SIZE: f32 = 12.0;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        ..Default::default()
    };
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::new(CalculatorApp::default()))
        }),
    )
}

struct CalculatorApp {
    calculator: Calculator,
    keypad: Keypad,
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // NOTE:singlelineは&strを渡すと、背景色が変わらなくなる。
            //      https://github.com/emilk/egui/blob/main/crates/egui/src/widgets/text_edit/builder.rs#L425
            ui.add_sized(
                [MONITOIR_WIDTH, MONITOR_HEIHGT],
                egui::TextEdit::singleline(&mut self.calculator.display_text())
                        .background_color(Color32::LIGHT_GRAY)
                        .text_color(Color32::BLACK)
                        .font(egui::FontId::proportional(FONT_SIZE))
                        .horizontal_align(egui::Align::Max)
                );
            
            
            ui.add_sized(
                [MONITOIR_WIDTH, MONITOR_HEIHGT],
                egui::TextEdit::singleline(&mut self.calculator.message())
                        .background_color(Color32::DARK_GRAY)
                        .text_color(Color32::BLACK)
                        .font(egui::FontId::proportional(FONT_SIZE))
                        .horizontal_align(egui::Align::Max)
                );

            self.keypad.add_ui(ui, &mut self.calculator);
        });
    }
}

impl Default for CalculatorApp {
    fn default() -> Self {
        CalculatorApp {
            calculator: Calculator::new(),
            keypad: Keypad::new(),
        }
    }
}
