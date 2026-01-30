use eframe::egui;
use crate::{game::Game, pieces::Color};

pub struct ChessApp {
    pub game: Game,
    selected: Option<(usize, usize)>,
    valid_moves: Vec<(usize, usize)>,
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            game: Game::new(),
            selected: None,
            valid_moves: vec![],
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
                        let is_selected = self.selected == Some((r, c));
                        let is_valid_move = self.valid_moves.contains(&(r, c));

                        let color = if is_selected {
                            egui::Color32::from_rgb(200, 150, 100)
                        } else if is_valid_move {
                            egui::Color32::from_rgb(150, 200, 100)
                        } else if dark {
                            egui::Color32::from_rgb(118, 150, 86)
                        } else {
                            egui::Color32::from_rgb(238, 238, 210)
                        };

                        let mut text = "".to_string();
                        if let Some(p) = self.game.board.get(r, c) {
                            text = p.to_char().to_string();
                        }

                        let btn = egui::Button::new(
                            egui::RichText::new(text)
                                .size(32.0)
                                .color(egui::Color32::BLACK)
                        )
                        .fill(color)
                        .min_size(egui::vec2(60.0, 60.0));

                        if ui.add(btn).clicked() {
                            if let Some(from) = self.selected {
                                if self.game.board.is_valid_move(from, (r, c), self.game.turn) {
                                    self.game.board.move_piece(from, (r, c));
                                    self.game.switch_turn();
                                }
                                self.selected = None;
                                self.valid_moves.clear();
                            } else {
                                self.selected = Some((r, c));
                                self.valid_moves = self.get_valid_moves((r, c));
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
                    self.selected = None;
                    self.valid_moves.clear();
                }

                if ui.button("reset").clicked() {
                    self.game = Game::new();
                    self.selected = None;
                    self.valid_moves.clear();
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label(format!("White Points: {}", self.game.board.get_white_points()));
                ui.label(format!("Black Points: {}", self.game.board.get_black_points()));
            });

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

impl ChessApp {
    fn get_valid_moves(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        for r in 0..8 {
            for c in 0..8 {
                if self.game.board.is_valid_move(pos, (r, c), self.game.turn) {
                    moves.push((r, c));
                }
            }
        }
        moves
    }
}
