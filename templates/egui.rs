use eframe::egui;

fn main() -> Result<(), eframe::Error>{
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(CalculatorApp::default()))),
    )
}


#[derive(Default)]
struct CalculatorApp {
    
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, World!");
        });
    }
}