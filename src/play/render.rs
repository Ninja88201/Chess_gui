use chess_lib::{Board, MoveList, Piece, Tile};
use egui::{Color32, ComboBox, Painter, Pos2, RichText, Ui, Vec2};

use crate::play::{state::PlayState, PlayTab};

impl PlayTab
{
    pub fn render_promotion_choices(
        &mut self,
        ui: &mut egui::Ui,
        _ctx: &egui::Context,
        tile: Tile,
        origin: Pos2,
    ) {
        if self.auto_queen {
            let _ = self.board.try_move_piece(self.selected.unwrap(), tile, Some(Piece::Queen));
            self.selected = None;
            self.state = PlayState::Playing(self.board.get_state());
            return;
        }
        let (x, y) = tile.get_coords();
        let rect = self.tile_to_screen(x as f32, y as f32, origin);
        let tile_size = self.board_size / 8.0;

        for (i, &piece) in Piece::PROMOTION_PIECES.iter().enumerate() {
            let pos_rect = rect.translate(Vec2::new(0.0, tile_size * i as f32));
            let response = ui.put(pos_rect, egui::Button::new("").corner_radius(0.0));

            let uv = self.atlas_uv(&piece, self.board.white_turn);
            let painter = ui.painter();

            painter.image(self.atlas.id(), pos_rect, uv, Color32::WHITE);

            if response.clicked() {
                let _ = self.board.try_move_piece(self.selected.unwrap(), tile, Some(piece));
                self.selected = None;
                self.state = PlayState::Playing(self.board.get_state());
            }
        }
    }

    pub fn render_engine_side_selector(&mut self, ui: &mut Ui) {
        let available_size = ui.available_size();

        let font_size = (available_size.x.min(available_size.y) * 0.1).clamp(12.0, 14.0);
        let combo_size = egui::vec2(
            available_size.x * 0.8, 
            available_size.y * 0.8
        );

        ui.vertical(|ui| {
            ui.label(RichText::new("Engine plays").size(font_size));

            ui.add_space(4.0);

            ComboBox::from_id_salt("engine_side_selector")
                .selected_text(match self.engine_plays {
                    Some(w) => if w { "White" } else { "Black" },
                    None => "Neither",
                })
                .width(combo_size.x)
                .height(combo_size.y)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.engine_plays, Some(true), RichText::new("White").size(font_size));
                    ui.selectable_value(&mut self.engine_plays, Some(false), RichText::new("Black").size(font_size));
                    ui.selectable_value(&mut self.engine_plays, None, RichText::new("Neither").size(font_size));
                });
            });
    }
    
    pub fn render_tiles(&self, painter: &Painter, origin: Pos2, board: &Board) {
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
                let mut clr   = if (file + rank) % 2 == 1 { light } else { dark };

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
    pub fn render_moves(&self, painter: &Painter, origin: Pos2, board: &Board) {
        if let Some(s) = self.selected {
            let mut moves = MoveList::new();
            board.generate_legal_moves_from(s, &mut moves);

            for m in moves.iter() {
                let (x, y) = m.to().get_coords();
                let rect = self.tile_to_screen(x as f32, y as f32, origin);

                let center = rect.center();

                painter.circle_filled(
                    center,
                    (self.board_size / 8.0) * 0.2,
                    Color32::from_rgba_unmultiplied(40, 40, 40, 180),
                );
            }
        }
        else {
            return;
        }
    }
}