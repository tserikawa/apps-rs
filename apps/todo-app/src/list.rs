use eframe::egui::Ui;
use eframe::egui::vec2;

pub fn add_list(ui: &mut Ui) {
    ui.horizontal(|ui| {
        let total_width = ui.available_width();
        let left_size = vec2(0.1 * total_width, ui.available_height());
        let middle_size = vec2(0.6 * total_width, ui.available_height());
        let right_size = vec2(0.3 * total_width, ui.available_height());

        ui.allocate_ui(left_size, |ui| {
            ui.checkbox(&mut false, String::new());
        });
        
        ui.allocate_ui(middle_size, |ui| {
            ui.text_edit_singleline(&mut "Hoge".to_string());
        });

        ui.allocate_ui(right_size, |ui| {
            _ = ui.button("Delete");
        });
    });
}