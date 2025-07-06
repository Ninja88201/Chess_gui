use chess_lib::Piece;
use egui::{Pos2, Rect, Vec2};

use crate::position_creator::PositionTab;



impl PositionTab
{
    pub fn tile_to_screen(&self, x: f32, y: f32, origin: Pos2) -> Rect {
        let ts = self.board_size / 8.0;
        let flipped_x = if self.flipped { 7.0 - x } else { x };
        let flipped_y = if self.flipped { y } else { 7.0 - y };
        let min = Pos2::new((flipped_x * ts) + origin.x, (flipped_y * ts) + origin.y);
        
        Rect::from_min_size(min, Vec2::splat(ts))
    }
    pub fn screen_to_tile(&self, pos: Pos2, origin: Pos2) -> (usize, usize) {
        let ts = self.board_size / 8.0;

        let mut file = ((pos.x - origin.x) / ts) as isize;
        let mut rank = ((pos.y - origin.y) / ts) as isize;

        // Clamp to valid range
        file = file.clamp(0, 7);
        rank = rank.clamp(0, 7);

        let (x, y) = if self.flipped {
            (7 - file as usize, rank as usize)
        } else {
            (file as usize, 7 - rank as usize)
        };

        (x, y)
    }
    pub fn atlas_uv(&self, piece: &Piece, colour: bool) -> Rect {
        // ------------  layout description  ----------------
        //   col: 0     1      2
        // row 0: WPawn WKnight WBishop
        // row 1: WRook WQueen  WKing
        // row 2: BPawn BKnight BBishop
        // row 3: BRook BQueen  BKing
        // -----------------------------------------------
        let (row, col) = match (piece, colour) {
            (Piece::Pawn  , true) => (0, 0),
            (Piece::Knight, true) => (0, 1),
            (Piece::Bishop, true) => (0, 2),
            (Piece::Rook  , true) => (1, 0),
            (Piece::Queen , true) => (1, 1),
            (Piece::King  , true) => (1, 2),
    
            (Piece::Pawn  , false) => (2, 0),
            (Piece::Knight, false) => (2, 1),
            (Piece::Bishop, false) => (2, 2),
            (Piece::Rook  , false) => (3, 0),
            (Piece::Queen , false) => (3, 1),
            (Piece::King  , false) => (3, 2),
        };
    
        let atlas_sz = self.atlas.size_vec2();
        let cell_w = atlas_sz.x / 3.0;
        let cell_h = atlas_sz.y / 4.0;
    
        let uv_min = Pos2::new(col as f32 * cell_w / atlas_sz.x, row as f32 * cell_h / atlas_sz.y);
        Rect::from_min_size(uv_min, Vec2::new(cell_w / atlas_sz.x, cell_h / atlas_sz.y))
    }
}