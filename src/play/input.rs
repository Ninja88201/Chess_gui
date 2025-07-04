use chess_lib::{Board, MoveList, MoveResult, Tile};
use egui::{InputState, Key, Pos2, Response};
use rand::Rng;

use crate::play::{state::PlayState, PlayTab};




impl PlayTab {
    pub fn move_input(&mut self, response: Response, origin: Pos2) {
        if !response.clicked() {
            return;
        }

        let pos = match response.interact_pointer_pos() {
            Some(pos) => pos,
            None => return,
        };

        let (x, y) = self.screen_to_tile(pos, origin);
        let player = self.board.current_players().0;

        let target_tile = match Tile::new_xy(x as u8, y as u8) {
            Some(t) => t,
            None => return,
        };

        if player.pieces.get_bit(target_tile) {
            // Selecting or deselecting a piece
            self.selected = match self.selected {
                Some(selected) if selected == target_tile => None,
                _ => Some(target_tile),
            };
            return;
        }

        // If we have a selected piece, try to move it
        let selected = match self.selected {
            Some(s) => s,
            None => return,
        };

        match self.board.try_move_piece(selected, target_tile, None) {
            Ok(move_result) => {
                match move_result {
                    MoveResult::MoveApplied(game_state) => {
                        self.state = PlayState::Playing(game_state);
                        self.selected = None;
                    }
                    MoveResult::PromotionNeeded(tile) => {
                        self.state = PlayState::Promotion(tile);
                    }
                }
            }
            Err(e) => {
                use chess_lib::MoveError as me;
                match e {
                    me::NoPieceSelected | me::FriendlyCapture | me::WrongTurn => {
                        unreachable!()
                    }
                    me::SameTile => {
                        // Unselect tile
                        self.selected = None;
                    }
                    me::IllegalMove => println!("That move is illegal"),
                    me::PiecePinned => {
                        // TODO: Flash King square or give visual feedback
                    }
                    me::Stalemate => println!("Stalemate"),
                    me::Checkmate => println!("You are in checkmate"),
                    me::Cancelled => {}
                }
            }
        }
    }
    pub fn utility_input(&mut self, input: &InputState) {

        // Reset Board
        if input.key_pressed(Key::R) {
            self.board = Board::new();
            self.view_board = Board::new();

            self.selected = None;
            self.state = PlayState::Playing(chess_lib::GameState::Playing);
        }

        // Flip Board
        if input.key_pressed(Key::F) {
            self.flipped = !self.flipped;
        }

        // Undo Move
        if input.modifiers.ctrl && input.key_pressed(Key::Z) {
            self.board.undo_move();

            self.selected = None;
            self.state = PlayState::Playing(chess_lib::GameState::Playing);
        }
        
        // Make random move
        if input.key_pressed(Key::Space) {
            let mut moves = MoveList::new();
            self.board.generate_legal_moves(self.board.white_turn, &mut moves);
            if !moves.is_empty() {
                let random_index = self.rand.random_range(0..moves.len());
                self.board.make_move_unchecked(moves[random_index]);
            }
            self.selected = None;
            self.state = PlayState::Playing(chess_lib::GameState::Playing);
        }

        // Exit program
        if input.key_pressed(Key::Escape) {
            self.should_close = true;
        }

        // View previous move
        if input.key_pressed(Key::ArrowRight) {
            if let PlayState::Viewing(pos) = self.state {
                self.state = PlayState::Viewing(pos + 1)
            }
        }

        // View next move
        if input.key_pressed(Key::ArrowLeft) {
            if let PlayState::Viewing(pos) = self.state {
                if pos != 0 {
                    self.state = PlayState::Viewing(pos - 1)
                }
            }
            else {
                if self.board.history.len() > 0 {
                    self.state = PlayState::Viewing(self.board.history.len() - 1)
                }
            }
        }
    }
    pub fn handle_play_state(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        response: egui::Response,
        origin: Pos2,
    ) {
        match self.state {
            PlayState::Viewing(_) => {
            }
            PlayState::Playing(game_state) => {
                use chess_lib::GameState as gs;
                match game_state {
                    gs::Playing => {
                        self.move_input(response, origin);
                    },
                    gs::Checkmate(_) => (),
                    gs::Stalemate(_) => (),
                    gs::InsufficientMaterial => (),
                    gs::FiftyMoveRule => (),
                    gs::ThreeRepetition => (),
                }
            }
            PlayState::Promotion(tile) => {
                self.render_promotion_choices(ui, ctx, tile, origin);
            }
        }
        ctx.input(|i| {
            self.utility_input(i);            
        });
    }
}
