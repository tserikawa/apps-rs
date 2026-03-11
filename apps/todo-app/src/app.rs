use eframe::egui::{CentralPanel, Context};
use eframe::Frame;

use crate::filter::{Filter, add_filter};
use crate::input::add_input;
use crate::list::add_list;
use crate::title::add_title;
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
            add_title(ui);
            add_input(ui, &mut self.input, &mut self.todos);
            add_filter(ui, &mut self.filter);
            add_list(ui, &mut self.todos, &self.filter);
        });
    }
}


