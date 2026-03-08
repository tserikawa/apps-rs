use eframe::egui;
use eframe::egui::Button;
use eframe::egui::Ui;
use eframe::egui::Vec2;
use eframe::egui::vec2;

use crate::calculator::Calculator;

pub const BUTTON_WIDTH: f32 = 40.0;
pub const BUTTON_HEIGHT: f32 = 40.0;

pub struct Keypad {
    id: egui::Id,
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            id: egui::Id::new("keypad"),
        }
    }

    pub fn add_ui(&self, ui: &mut Ui, calculator: &mut Calculator) {
        let window_margin = ui.spacing().window_margin;
        let size_1x1 = vec2(BUTTON_WIDTH, BUTTON_HEIGHT);
        
        ui.spacing_mut().item_spacing = Vec2::splat(window_margin.leftf());

        ui.horizontal(|ui| {
            if ui.add_sized(size_1x1, Button::new("1")).clicked() {
                calculator.add_term("1");
            }
            if ui.add_sized(size_1x1, Button::new("2")).clicked() {
                calculator.add_term("2");
            }
            if ui.add_sized(size_1x1, Button::new("3")).clicked() {
                calculator.add_term("3");
            }
            if ui.add_sized(size_1x1, Button::new("+")).clicked() {
                calculator.add_term("+");
            }
        });
        ui.horizontal(|ui| {
            if ui.add_sized(size_1x1, Button::new("4")).clicked() {
                calculator.add_term("4");
            }
            if ui.add_sized(size_1x1, Button::new("5")).clicked() {
                calculator.add_term("5");
            }
            if ui.add_sized(size_1x1, Button::new("6")).clicked() {
                calculator.add_term("6");
            }
            if ui.add_sized(size_1x1, Button::new("-")).clicked() {
                calculator.add_term("-");
            }
        });
        ui.horizontal(|ui| {
            if ui.add_sized(size_1x1, Button::new("7")).clicked() {
                calculator.add_term("7");
            }
            if ui.add_sized(size_1x1, Button::new("8")).clicked() {
                calculator.add_term("8");
            }
            if ui.add_sized(size_1x1, Button::new("9")).clicked() {
                calculator.add_term("9");
            }
            if ui.add_sized(size_1x1, Button::new("*")).clicked() {
                calculator.add_term("*");
            }
        });
        ui.horizontal(|ui| {
            if ui.add_sized(size_1x1, Button::new("0")).clicked() {
                calculator.add_term("0");
            }
            if ui.add_sized(size_1x1, Button::new("C")).clicked() {
                calculator.clear();
            }
            if ui.add_sized(size_1x1, Button::new("=")).clicked() {
                match calculator.result() {
                    Ok(result) => calculator.set_result(result),
                    Err(e) => {
                        calculator.set_message(&format!("{e}"));
                        calculator.clear();
                    }
                }
            }
            if ui.add_sized(size_1x1, Button::new("/")).clicked() {
                calculator.add_term("/");
            }
        });
    }
}

impl Default for Keypad {
    fn default() -> Self {
        Self::new()
    }
}
