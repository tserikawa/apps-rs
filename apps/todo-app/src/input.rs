use eframe::egui::{Button, Color32, FontFamily, FontId, Stroke, TextEdit, TextStyle, Ui, vec2};

pub fn add_input(ui: &mut Ui) {
    ui.horizontal(|ui| {
        // horizontal全体のBodyフォントサイズを変更
        ui.style_mut().text_styles.insert(
            TextStyle::Body,
            FontId::new(18.0, FontFamily::Proportional),
        );
        ui.style_mut().text_styles.insert(
            TextStyle::Button,
            FontId::new(18.0, FontFamily::Proportional),
        );

        let total_width = ui.available_width();
        let left_width = 0.7 * total_width;
        let right_width = 0.3 * total_width;

        ui.allocate_ui(vec2(left_width, ui.available_height()), |ui| {
            ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::RED);
            ui.add(TextEdit::singleline(&mut String::new()));
            ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::TRANSPARENT);
        });

        ui.allocate_ui(vec2(right_width, ui.available_height()), |ui| {
            ui.add(
                Button::new("Add")
            );
        });
    });
}