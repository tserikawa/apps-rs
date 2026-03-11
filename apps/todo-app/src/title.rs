use eframe::egui::{Align, Layout, RichText, Ui};

pub fn update_title(ui: &mut Ui) {
    ui.with_layout(
        Layout::top_down(Align::Center),
        |ui| {
            ui.label(RichText::new("ToDo").size(20.0));
        }
    );
}
