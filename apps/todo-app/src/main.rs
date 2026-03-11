use app::ToDoApp;
use eframe::egui::{Theme, ViewportBuilder};
use eframe::{self, run_native};
use eframe::{Error, NativeOptions};

mod app;
mod filter;
mod input;
mod title;
mod list;
mod todo;

fn main() -> Result<(), Error> {
    let windows_size = [400.0, 500.0];
    let viewport = ViewportBuilder::default()
        .with_inner_size(windows_size) // 初期サイズ
        .with_min_inner_size(windows_size) // 最小サイズ
        .with_max_inner_size(windows_size) // 最大サイズ
        .with_resizable(false); // サイズ変更
    let options = NativeOptions {
        viewport,
        ..Default::default()
    };

    run_native(
        "ToDo App",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(Theme::Light); // 表示モード
            let app = Box::new(ToDoApp::default());
            Ok(app)
        }),
    )
}
