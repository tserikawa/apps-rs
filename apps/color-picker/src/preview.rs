use eframe::egui;
use eframe::egui::{Align, Layout, RichText, Ui};

pub fn update_preview(ui: &mut Ui) {
    ui.with_layout(
        Layout::top_down(Align::Center), 
        |ui| {
            // 1. 領域を確保する
            let (rect, _response) = ui.allocate_exact_size(
                egui::vec2(360.0, 120.0),  // 幅×高さ
                egui::Sense::empty(),
            );

            // 2. Painterを取得する
            let painter = ui.painter();

            // 3. 確保した領域を指定色で塗りつぶす
            painter.rect_filled(
                rect,
                8.0,  // 角丸の半径
                egui::Color32::from_rgb(255, 0, 0),
            );

            // 4. 枠線を描画する
            painter.rect_stroke(
                rect,
                8.0,
                egui::Stroke::new(1.0, egui::Color32::DARK_GRAY),
                egui::StrokeKind::Middle
            );
        }
    );

    ui.with_layout(
        Layout::top_down(Align::Center),
        |ui| {
            ui.label(RichText::new("HEX:FF5733").size(18.0));
        }
    );
}