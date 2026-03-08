use eframe::egui::Ui;
use eframe::egui::vec2;

#[derive(PartialEq)]
pub enum Filter {
    All,
    NotCompleted,
    Completed
}

impl Default for Filter {
    fn default() -> Self {
        Self::All
    }
}

pub fn add_filter(ui: &mut Ui, filter: &mut Filter) {
    ui.horizontal(|ui| {
        let total_width = ui.available_width();
        let left_size = vec2(0.3 * total_width, ui.available_height());
        let middle_size = vec2(0.3 * total_width, ui.available_height());
        let right_size = vec2(0.3 * total_width, ui.available_height());

        ui.allocate_ui(left_size, |ui| {
            ui.radio_value(filter, Filter::All, "All");
        });

        ui.allocate_ui(middle_size, |ui| {
            ui.radio_value(filter, Filter::NotCompleted, "NotCompleted");
        });

        ui.allocate_ui(right_size, |ui| {
            ui.radio_value(filter, Filter::Completed, "Completed");
        });
    });
}