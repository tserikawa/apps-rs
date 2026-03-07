use eframe::egui::{self, Color32};

use crate::{calculator::Calculator, keypad::Keypad};

mod calculator;
mod keypad;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size([160.0, 180.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Calculator",
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
            egui::TextEdit::singleline(&mut self.calculator.display_text())
                .background_color(Color32::LIGHT_GRAY)
                .text_color(Color32::BLACK)
                .horizontal_align(egui::Align::Max)
                .show(ui);

            egui::TextEdit::singleline(&mut self.calculator.message())
                .background_color(Color32::DARK_GRAY)
                .text_color(Color32::BLACK)
                .horizontal_align(egui::Align::Max)
                .show(ui);
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
