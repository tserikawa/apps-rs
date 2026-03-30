use eframe::egui;
use eframe::egui::Ui;


pub fn update_slider(ui: &mut Ui) {
    let mut r = 0.0;
    ui.horizontal(|ui| {
        ui.label("R");
        ui.add(egui::Slider::new(&mut r, 0.0..=100.0).text("My value"));
    });
}