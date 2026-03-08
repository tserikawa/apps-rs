use eframe::egui::{self};

use crate::{
    calculator::Calculator,
    display::{DISPLAY_HEIHGT, Display},
    keypad::{BUTTON_HEIGHT, BUTTON_WIDTH, Keypad},
};

mod calculator;
mod decoder;
mod display;
mod keypad;

pub const APP_NAME: &str = "Calculator";
pub const MARGIN: f32 = 6.0;
pub const WINDOW_WIDTH: f32 = BUTTON_WIDTH * 4.0 + MARGIN * 5.0;
pub const WINDOW_HEIGHT: f32 = BUTTON_HEIGHT * 4.0 + MARGIN * 7.0 + 2.0 * DISPLAY_HEIHGT;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
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
    display: Display,
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // NOTE:singlelineは&strを渡すと、背景色が変わらなくなる。
            //      https://github.com/emilk/egui/blob/main/crates/egui/src/widgets/text_edit/builder.rs#L425
            self.display
                .add_main_display(ui, &mut self.calculator.display_text());
            self.display
                .add_message_display(ui, &mut self.calculator.message());
            self.keypad.add_ui(ui, &mut self.calculator);
        });
    }
}

impl Default for CalculatorApp {
    fn default() -> Self {
        CalculatorApp {
            calculator: Calculator::new(),
            keypad: Keypad::new(),
            display: Display,
        }
    }
}
