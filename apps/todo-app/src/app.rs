use eframe::egui::{CentralPanel, Context};
use eframe::Frame;

use crate::filter::{Filter, add_filter};
use crate::input::add_input;
use crate::title::add_title;

#[derive(Default)]
pub struct ToDoApp{
    filter: Filter,
}

impl eframe::App for ToDoApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            add_title(ui);
            add_input(ui);
            add_filter(ui, &mut self.filter);
        });
    }
}


