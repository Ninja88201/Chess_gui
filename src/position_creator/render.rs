use chess_lib::{Board, Piece, Tile};
use egui::{Color32, Context, Painter, Pos2, RichText, Vec2};

use crate::position_creator::PositionTab;


impl PositionTab
{
    pub fn render_board(&mut self, ctx: &Context) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
            let available_size = ui.available_size();


            self.board_size = available_size.x.min(available_size.y) * 0.8;

            let horizontal_margin = (available_size.x - self.board_size) / 2.0;
            let vertical_margin = (available_size.y - self.board_size) / 2.0;

            if vertical_margin > 0.0 {
                ui.add_space(vertical_margin);
            }

            ui.horizontal(|ui| {
                if horizontal_margin > 0.0 {
                    ui.add_space(horizontal_margin);
                }

                let (response, painter) = ui.allocate_painter(
                    Vec2::splat(self.board_size),
                    egui::Sense::click_and_drag(),
                );

                if response.dragged_by(egui::PointerButton::Secondary) || response.secondary_clicked() {
                    let pos = response.interact_pointer_pos();
                    if let Some(p) = pos {
                        let (x, y) = self.screen_to_tile(p, response.rect.min);
                        let t = Tile::new_xy(x as u8, y as u8);
                        if let Some(tile) = t {
                            if let Some((p, w)) = self.board.get_piece_at_tile(tile) {
                                if w {
                                    self.board.white.remove_piece_type(p, tile);
                                } else {
                                    self.board.black.remove_piece_type(p, tile);
                                }
                            }
                        }
                    }
                }
                if response.dragged_by(egui::PointerButton::Primary) || response.clicked() {
                    let pos = response.interact_pointer_pos();
                    if let Some(p) = pos {
                        let (x, y) = self.screen_to_tile(p, response.rect.min);
                        let t = Tile::new_xy(x as u8, y as u8);
                        if let Some(tile) = t {
                            if let Some((piece, colour)) = self.selected_piece {
                                if colour {
                                    self.board.white.place_piece(piece, tile);
                                } else {
                                    self.board.black.place_piece(piece, tile);
                                }
                            } else { 
                                if let Some((p, w)) = self.board.get_piece_at_tile(tile) {
                                    if w {
                                        self.board.white.remove_piece_type(p, tile);
                                    } else {
                                        self.board.black.remove_piece_type(p, tile);
                                    }
                                }
                            }
                        }

                    }
                        
                }
                
                let origin = response.rect.min;

                self.render_tiles(&painter, origin);
                self.render_pieces(&painter, origin, &self.board);


                if horizontal_margin > 0.0 {
                    ui.add_space(horizontal_margin);
                }
            });

            ui.add_space(12.0);

            ui.vertical_centered(|ui| {
                let font_size = self.board_size * 0.02;
                let fen_text = RichText::new(format!("FEN: {}", self.board.to_fen()))
                    .monospace()
                    .size(font_size);
                ui.label(fen_text);

                ui.add_space(8.0);
            });
        });
    }
    pub fn render_tiles(&self, painter: &Painter, origin: Pos2) {
        // Draw board

        for rank in 0..8 {
            for file in 0..8 {
                let rect = self.tile_to_screen(file as f32, rank as f32, origin);
                let light = Color32::from_rgb(240, 217, 181);
                let dark  = Color32::from_rgb(181, 136,  99);
                let clr   = if (file + rank) % 2 == 1 { light } else { dark };
                painter.rect_filled(rect, 0.0, clr);

            }
        }
    }
    pub fn render_pieces(&self, painter: &Painter, origin: Pos2, board: &Board) {
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
}