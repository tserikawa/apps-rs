use eframe::egui::Ui;
use eframe::egui::vec2;

use crate::filter::Filter;
use crate::todo::ToDoCollection;

pub fn update_list(ui: &mut Ui, todo_collection: &mut ToDoCollection, filter: &Filter) {
    let mut delete_ids = Vec::new();

    let items = todo_collection.items_mut(filter);
    for item in items {
        ui.horizontal(|ui| {
            let total_width = ui.available_width();
            let left_size = vec2(0.1 * total_width, ui.available_height());
            let middle_size = vec2(0.6 * total_width, ui.available_height());
            let right_size = vec2(0.3 * total_width, ui.available_height());

            ui.allocate_ui(left_size, |ui| {
                ui.checkbox(&mut item.completed, String::new());
            });
            
            ui.allocate_ui(middle_size, |ui| {
                ui.text_edit_singleline(&mut item.content);
            });

            ui.allocate_ui(right_size, |ui| {
                if ui.button("Delete").clicked() {
                    delete_ids.push(item.id);
                }
            });
        });
    }
    
    todo_collection.delete(&delete_ids);
}