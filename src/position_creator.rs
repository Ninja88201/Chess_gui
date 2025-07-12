use chess_lib::{Board, CastlingRights, Colour, Piece};
use egui::{load::SizedTexture, Color32, ComboBox, Context, Image, ImageButton, Key, TextureHandle, Ui, Vec2};

mod helper;
mod render;

pub struct PositionTab
{
    pub board: Board,

    pub board_size: f32,
    pub flipped: bool,
    pub selected_piece: Option<(Piece, bool)>,
    pub inputted_fen: String,

    pub atlas: TextureHandle,
    
    pub should_close: bool,
    pub change_tab: bool,
}

impl PositionTab
{
    pub fn new(position: Option<&str>, atlas: TextureHandle) -> Self {
        let board = match position {
            Some(pos) => Board::new_from_fen(pos).unwrap(),
            None => Board::new(),
        };
        Self { 
            
            board: board, 

            board_size: 400.0,
            flipped: false,
            selected_piece: None,
            inputted_fen: String::new(),

            atlas,

            should_close: false,
            change_tab: false,
        }
    }
    pub fn render(&mut self, ctx: &Context) {
        ctx.input(|i| {
            if i.key_pressed(Key::Escape) {
                self.should_close = true;
            }
        });
        let screen_size = ctx.screen_rect();
        let is_portrait = screen_size.height() > screen_size.width();

        self.render_board(ctx);
        self.render_side_panel(ctx);
    }
    pub fn render_side_panel(&mut self, ctx: &Context) {
        egui::SidePanel::right("piece_selection")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Pieces");
                self.render_piece_buttons(ui);
                ui.separator();
                self.render_helper_buttons(ui);
            });
    }

    pub fn render_piece_buttons(&mut self, ui: &mut Ui) {
        let button_size = 50.0;

        ui.horizontal(|ui| {
            // Render white pieces
            ui.vertical(|ui| {
                for piece in Piece::ALL_PIECES {
                    self.render_piece_button(ui, piece, true, button_size);
                }
            });

            // Render black pieces
            ui.vertical(|ui| {
                for piece in Piece::ALL_PIECES {
                    self.render_piece_button(ui, piece, false, button_size);
                }
            });
        });
    }

    /// Renders a single piece selection button.
    fn render_piece_button(&mut self, ui: &mut Ui, piece: Piece, is_white: bool, size: f32) {
        let uv_rect = self.atlas_uv(&piece, is_white);

        let is_selected = matches!(self.selected_piece, Some((p, w)) if p == piece && w == is_white);

        let color = if is_selected {
            Color32::GRAY
        } else {
            Color32::DARK_GRAY
        };

        let button = ImageButton::new(
            Image::from_texture(Into::<SizedTexture>::into(&self.atlas))
                .uv(uv_rect)
                .max_size(Vec2::splat(size))
                .bg_fill(color),
        );

        let response = ui.add(button).interact(egui::Sense::click());
        if response.clicked() {
            if is_selected {
                self.selected_piece = None; // Deselect if already selected
            } else {
                self.selected_piece = Some((piece, is_white)); // Otherwise select
            }
        }
    }
    pub fn render_helper_buttons(&mut self, ui: &mut Ui) {
        ui.heading("Board Settings");

        ui.add_space(8.0);

        if ui.button("Clear board").clicked() {
            self.board = Board::new_empty();
        }

        ui.add_space(8.0);
        
        if ui.button("Reset board").clicked() {
            self.board = Board::new();
        }

        ui.add_space(8.0);

        ui.label("Select board turn");
        let text = if self.board.turn.white() { "White" } else { "Black" };
        ComboBox::from_id_salt("board_turn")
            .selected_text(text)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.board.turn, Colour::White, "White");
                ui.selectable_value(&mut self.board.turn, Colour::Black, "Black");
            });

        ui.add_space(8.0);

        ui.label("White castling");
        self.castling_right_checkbox(ui, CastlingRights::WHITE_KINGSIDE, "Kingside");
        self.castling_right_checkbox(ui, CastlingRights::WHITE_QUEENSIDE, "Queenside");

        ui.add_space(4.0);

        ui.label("Black castling");
        self.castling_right_checkbox(ui, CastlingRights::BLACK_KINGSIDE, "Kingside");
        self.castling_right_checkbox(ui, CastlingRights::BLACK_QUEENSIDE, "Queenside");

        ui.add_space(8.0);

        if ui.button("Play position").clicked() {
            self.change_tab = true;
        }

        ui.add_space(8.0);

        ui.text_edit_singleline(&mut self.inputted_fen);
        if ui.button("Load FEN position").clicked() {
            let b = Board::new_from_fen(&self.inputted_fen);
            match b {
                Ok(board) => self.board = board,
                Err(e) => (),
            }
        }
    }

    fn castling_right_checkbox(&mut self, ui: &mut Ui, right: CastlingRights, label: &str) {
        let mut enabled = self.board.castling.contains(right);
        if ui.checkbox(&mut enabled, label).changed() {
            if enabled {
                self.board.castling.insert(right);
            } else {
                self.board.castling.remove(right);
            }
        }
    }

}