use eframe::egui::{CentralPanel, Context};
use eframe::Frame;

use crate::filter::{Filter, update_filter};
use crate::input::update_input;
use crate::list::update_list;
use crate::title::update_title;
use crate::todo::ToDoCollection;

#[derive(Default)]
pub struct ToDoApp{
    filter: Filter,
    input: String,
    todos: ToDoCollection
}

impl eframe::App for ToDoApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            update_title(ui);
            update_input(ui, &mut self.input, &mut self.todos);
            update_filter(ui, &mut self.filter);
            update_list(ui, &mut self.todos, &self.filter);
        });
    }
}


