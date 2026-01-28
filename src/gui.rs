use eframe::egui;
use crate::{game::Game, pieces::Color};

pub struct ChessApp {
    pub game: Game,
    selected: Option<(usize, usize)>,
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            game: Game::new(),
            selected: None,
        }
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("♟ chess");
            ui.separator();

            ui.label(format!(
                "Turn: {}",
                if self.game.turn == Color::White { "White ♙" } else { "Black ♟" }
            ));

            ui.add_space(10.0);

            egui::Grid::new("board").spacing([0.0, 0.0]).show(ui, |ui| {
                for r in (0..8).rev() {
                    for c in 0..8 {
                        let dark = (r + c) % 2 == 1;
                        let color = if dark {
                            egui::Color32::from_rgb(118,150,86)
                        } else {
                            egui::Color32::from_rgb(238,238,210)
                        };

                        let mut text = "".to_string();
                        if let Some(p) = self.game.board.get(r,c) {
                            text = p.to_char().to_string();
                        }

                        let btn = egui::Button::new(
                            egui::RichText::new(text)
                                .size(32.0)
                                .color(egui::Color32::BLACK)
                        )
                        .fill(color)
                        .min_size(egui::vec2(60.0,60.0));

                        if ui.add(btn).clicked() {
                            if let Some(from) = self.selected {
                                self.game.board.move_piece(from,(r,c));
                                self.game.switch_turn();
                                self.selected = None;
                            } else {
                                self.selected = Some((r,c));
                            }
                        }
                    }
                    ui.end_row();
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("new game").clicked() {
                    self.game = Game::new();
                }

                if ui.button("reset").clicked() {
                    self.game = Game::new();
                }
            });

            ui.separator();

            ui.label("White captured:");
            ui.horizontal(|ui| {
                for p in &self.game.board.captured_black {
                    ui.label(p.to_char().to_string());
                }
            });

            ui.label("Black captured:");
            ui.horizontal(|ui| {
                for p in &self.game.board.captured_white {
                    ui.label(p.to_char().to_string());
                }
            });
        });
    }
}
