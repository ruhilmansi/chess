mod board;
mod game;
mod pieces;
mod gui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "rust chess",
        options,
        Box::new(|_| Box::new(gui::ChessApp::default())),
    )
}
