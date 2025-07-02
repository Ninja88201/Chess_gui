use chess_lib::{Board, MoveList, Piece};
use egui::{Button, Key, RichText};
use egui::{Color32, Context, Painter, Pos2, Vec2};

use crate::app::ChessApp;
use crate::app::UIState;

impl ChessApp {
    pub fn render_tiles(&self, board: &Board, painter: &Painter, origin: Pos2) {
        // Draw board
        let white_check = board.is_in_check(true);
        let black_check = board.is_in_check(false);

        let w_king = board.white.king_tile();
        let b_king = board.black.king_tile();
        for rank in 0..8 {
            for file in 0..8 {
                let rect = self.tile_to_screen(file as f32, rank as f32, origin);
                let light = Color32::from_rgb(240, 217, 181);
                let dark  = Color32::from_rgb(181, 136,  99);
                let mut clr   = if (file + rank) % 2 == 0 { light } else { dark };

                if white_check {
                    if w_king.get_coords() == (file, rank) {
                        clr = clr.blend(Color32::RED);
                    }
                }
                if black_check {
                    if b_king.get_coords() == (file, rank) {
                        clr = clr.blend(Color32::RED);
                    }
                }
                painter.rect_filled(rect, 0.0, clr);

            }
        }
    }
    pub fn render_pieces(&self, board: &Board, painter: &Painter, origin: Pos2) {
        for (is_white, player) in [(true, &board.white), (false, &board.black)] {
            for (i, bb) in player.bb.iter().enumerate() {
                let piece = Piece::from_index(i);
                let uv_rect = self.atlas_uv(&piece, is_white);
                for t in bb.iter() {
                    let (x, y) = t.get_coords();
                    painter.image(
                        self.atlas.id(), 
                        self.tile_to_screen(x as f32, y as f32, origin), 
                        uv_rect, 
                        Color32::WHITE,
                    );
                }
            }
        }
    }
    pub fn render_moves(&self, painter: &Painter, origin: Pos2) {
        if let Some(s) = self.selected {
            let mut moves = MoveList::new();
            self.board.generate_legal_moves_from(s, &mut moves);

            for m in moves.iter() {
                let (x, y) = m.to().get_coords();
                let rect = self.tile_to_screen(x as f32, y as f32, origin);

                let center = rect.center();
                let radius = self.tile_size() * 0.2;

                painter.circle_filled(
                    center,
                    radius,
                    Color32::from_rgba_unmultiplied(40, 40, 40, 180),
                );
            }
        }
        else {
            return;
        }
    }
    pub fn render_board(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Get available space and choose the smaller dimension (square board)
            let available_size = ui.available_size();
            self.board_size = available_size.x.min(available_size.y) * 0.9;

            let (response, painter) = ui.allocate_painter(
                Vec2::splat(self.board_size),
                egui::Sense::click(),
            );

            let origin = response.rect.min;

            if let UIState::Viewing(pos) = self.ui_state {
                let current = self.board.history.len();
                if pos == current {
                    self.ui_state = UIState::Playing;
                }
                let mut board = self.board.clone();
                for _ in 0..(current - pos) {
                    board.undo_move();
                }
                self.render_tiles(&board, &painter, origin);
                self.render_pieces(&board, &painter, origin);
            }
            else {
                self.render_tiles(&self.board, &painter, origin);
                self.render_pieces(&self.board, &painter, origin);
                self.render_moves(&painter, origin);
            }

            ui.add_space(10.0);

            // Set font size relative to board size
            let font_size = self.board_size * 0.02; // tweak factor (e.g., 3% of board size)
            let text = RichText::new(format!("FEN: {}", self.board.to_fen()))
                .monospace()
                .size(font_size);
            ui.label(text);


            match self.ui_state {
                UIState::Viewing(pos) => {
                    ctx.input(|i| {
                        self.utility_input(i, ctx);
                    });
                },
                UIState::Playing => {
                    self.move_input(response, origin);

                    ctx.input(|i| {
                        self.utility_input(i, ctx);
                    });
                },
                UIState::Promotion(tile) => {
                    let (x, y) = tile.get_coords();
                    let rect = self.tile_to_screen(x as f32, y as f32, origin);
                    let tile_size = self.tile_size();

                    for (i, &piece) in Piece::PROMOTION_PIECES.iter().enumerate() {
                        let pos_rect = rect.translate(Vec2::new(0.0, tile_size * i as f32));

                        let response = ui.put(pos_rect, Button::new("").corner_radius(0.0));

                        let uv = self.atlas_uv(&piece, self.board.white_turn);
                        let painter = ui.painter();

                        painter.image(
                            self.atlas.id(),
                            pos_rect,
                            uv,
                            Color32::WHITE,
                        );

                        if response.clicked() {
                            let _ = self.board.try_move_piece(self.selected.unwrap(), tile, Some(piece));
                            self.selected = None;
                            self.ui_state = UIState::Playing;
                        }
                    }
                    ctx.input(|i| {
                        self.utility_input(i, ctx);
                    });
                },
                UIState::Checkmate(w) => {

                },
                UIState::Stalemate(w) => {

                },
                UIState::FiftyMoveRule => {

                },
            }

        });
    }
}