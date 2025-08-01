use chess_lib::{Board, Colour, MoveList, Piece, Tile};
use egui::{Color32, ComboBox, Context, Painter, Pos2, RichText, Vec2};
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;

use crate::play::{state::{Engine, PlayState}, PlayTab};

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

            let uv = self.atlas_uv(&piece, self.board.turn);
            let painter = ui.painter();

            painter.image(self.atlas.id(), pos_rect, uv, Color32::WHITE);

            if response.clicked() {
                let _ = self.board.try_move_piece(self.selected.unwrap(), tile, Some(piece));
                self.selected = None;
                self.state = PlayState::Playing(self.board.get_state());
            }
        }
    }

   pub fn render_engine_side_selector(&mut self, ui: &mut egui::Ui) {
    let available_size = ui.available_size();

    let font_size = (available_size.x.min(available_size.y) * 0.1).clamp(12.0, 14.0);
    let combo_size = egui::vec2(
        available_size.x * 0.8,
        available_size.y * 0.8,
    );


    ui.vertical(|ui| {
        ui.label(RichText::new("Engine plays").size(font_size));
        ui.add_space(4.0);

        ComboBox::from_id_salt("engine_side_selector")
            .selected_text(self.engine_plays.to_string())
            .width(combo_size.x)
            .height(combo_size.y)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.engine_plays, Engine::White, RichText::new("White").size(font_size));
                ui.selectable_value(&mut self.engine_plays, Engine::Black, RichText::new("Black").size(font_size));
                ui.selectable_value(&mut self.engine_plays, Engine::Neither, RichText::new("Neither").size(font_size));
                ui.selectable_value(&mut self.engine_plays, Engine::Both, RichText::new("Both").size(font_size));
            });

        // Countdown display
        if self.engine_turn() {
                let remaining = (self.seconds_per_move - self.engine_timer).max(0.0);
                let progress = (self.engine_timer / self.seconds_per_move).clamp(0.0, 1.0);

                ui.add_space(8.0);
                ui.label(RichText::new(format!("Next move in: {:.1}s", remaining)).size(font_size - 1.0));
                ui.add(egui::ProgressBar::new(progress).desired_width(combo_size.x));
            }
    });
}
    
    pub fn render_tiles(&self, painter: &Painter, origin: Pos2, board: &Board) {
        // Draw board
        let white_check = board.is_in_check(Colour::White);
        let black_check = board.is_in_check(Colour::Black);

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
        for (colour, player) in [(Colour::White, &board.white), (Colour::Black, &board.black)] {
            for (i, bb) in player.bb.iter().enumerate() {
                let piece = Piece::from_index(i);
                let uv_rect = self.atlas_uv(&piece, colour);
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
    pub fn render_game_over(&mut self, ctx: &Context) {
        if let PlayState::Playing(game_state) = self.state {
            match game_state {
                chess_lib::GameState::Playing => (),
                chess_lib::GameState::Checkmate(_) => self.show_popup = true,
                chess_lib::GameState::Stalemate(_) => self.show_popup = true,
                chess_lib::GameState::InsufficientMaterial => self.show_popup = true,
                chess_lib::GameState::FiftyMoveRule => self.show_popup = true,
                chess_lib::GameState::ThreeRepetition => self.show_popup = true,
            };
            if !self.show_popup { return; }
            let message = match game_state {
                chess_lib::GameState::Checkmate(loser) => format!("Checkmate! {} wins.", if loser.white() { "Black" } else { "White" } ),
                chess_lib::GameState::Stalemate(_) => "Stalemate! It's a draw.".to_string(),
                chess_lib::GameState::InsufficientMaterial => "Draw: Insufficient material.".to_string(),
                chess_lib::GameState::FiftyMoveRule => "Draw: 50-move rule.".to_string(),
                chess_lib::GameState::ThreeRepetition => "Draw: Threefold repetition.".to_string(),
                _ => return,
            };
            egui::Window::new("Game Over")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .frame(egui::Frame::popup(&ctx.style()))
                .default_width(200.0)
                .min_width(0.0)
                .default_height(0.0)  
                .min_height(0.0)          
                .show(ctx, |ui| {
                    ui.label(message);
                    ui.add_space(10.0);

                    ui.with_layout(
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            if ui.button("New Game").clicked() {
                                self.reset();
                                self.show_popup = false;
                            }

                            ui.add_space(8.0);

                            if ui.button("Save Game").clicked() {
                                let pgn = self.board.to_pgn();

                                #[cfg(target_arch = "wasm32")]
                                Self::download_pgn_web(&pgn);

                                #[cfg(not(target_arch = "wasm32"))]
                                Self::download_pgn_native(&pgn);
                            }
                        },
                    );
                });
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn download_pgn_web(pgn: &str) {
        use web_sys::wasm_bindgen::JsCast;
        use web_sys::{Blob, Url, HtmlAnchorElement};

        let array = js_sys::Array::new();
        array.push(&web_sys::wasm_bindgen::JsValue::from_str(pgn));

        let blob = Blob::new_with_str_sequence(&array).unwrap();
        let url = Url::create_object_url_with_blob(&blob).unwrap();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let a = document
            .create_element("a")
            .unwrap()
            .unchecked_into::<HtmlAnchorElement>();

        a.set_href(&url);
        a.set_download("game.pgn");
        a.click();

        Url::revoke_object_url(&url).ok();
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn download_pgn_native(pgn: &str) {
        if let Some(path) = FileDialog::new()
            .set_file_name("game.pgn")
            .add_filter("PGN", &["pgn"])
            .save_file()
        {
            if let Err(e) = fs::write(&path, pgn) {
                eprintln!("Failed to save PGN file: {}", e);
            }
        }
    }
}