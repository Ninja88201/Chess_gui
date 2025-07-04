use std::{ops::RangeInclusive, time::Instant};

use chess_engine::search::find_best_move;
use chess_lib::{Board, Tile};
use egui::{Context, Layout, RichText, Slider, TextureHandle, Ui, Vec2};
use rand::rngs::ThreadRng;

mod helper;
mod render;
mod state;
use state::PlayState;

use crate::play::state::Engine;
mod input;

pub struct PlayTab
{
    pub board: Board,
    pub view_board: Board,

    pub rand: ThreadRng,
    
    pub flipped: bool,
    pub selected: Option<Tile>,

    pub engine_plays: Engine,
    pub auto_queen: bool,
    last_frame_time: Instant,
    engine_timer: f32,
    pub seconds_per_move: f32,

    pub split_ratio: f32,

    pub atlas: TextureHandle,
    pub board_size: f32,

    pub state: PlayState,
    pub show_popup: bool,
    pub should_close: bool,
    
}
impl PlayTab
{
    pub fn new(position: Option<&str>, atlas: TextureHandle) -> Self {
        let board = match position {
            Some(fen) => Board::new_from_fen(fen).unwrap(),
            None => Board::new(),
        };
        Self {
            board: board.clone(),
            view_board: board,

            rand: rand::rng(),

            flipped: false,
            selected: None,
            
            engine_plays: Engine::Neither,
            auto_queen: false,
            last_frame_time: Instant::now(),
            engine_timer: 0.0,
            seconds_per_move: 1.0,

            split_ratio: 0.5,

            atlas: atlas,
            board_size: 400.0,

            state: PlayState::Playing(chess_lib::GameState::Playing),
            show_popup: true,
            should_close: false,

        }

    }
    pub fn resest(&mut self) {
        self.board = Board::new();
        self.view_board = Board::new();

        self.state = PlayState::Playing(chess_lib::GameState::Playing);
        self.show_popup = true;
    }
    pub fn engine_turn(&self) -> bool {
        let engine = self.engine_plays;
        if engine == Engine::Neither {
            return false;
        }
        if engine == Engine::Both {
            return true;
        }
        let turn = self.board.white_turn;
        if (turn && engine == Engine::White) || (!turn && engine == Engine::Black) {
            return true;
        }
        unreachable!()
    }
    pub fn render(&mut self, ctx: &Context) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;

        self.engine_timer += dt;

        if self.engine_turn()  {
            if self.engine_timer >= self.seconds_per_move {
                self.engine_timer = 0.0;

                if let Some(m) = find_best_move(&mut self.board, 4) {
                    self.board.make_move_unchecked(m);
                }
            } else {
                ctx.request_repaint();
            }
        }

        // Modify view board to correct position
        if let PlayState::Viewing(pos) = self.state {
            let curr_pos = self.view_board.history.len();
            if self.board.history.len() == pos {
                self.state = PlayState::Playing(self.board.get_state());
            }
            
            if curr_pos > pos {
                let delta = curr_pos - pos;
                for _ in 0..delta {
                    self.view_board.undo_move();
                }
            } else {
                let delta = pos - curr_pos;
                for i in 0..delta {
                    if let Some(&mv) = self.board.history.get(curr_pos + i) {
                        self.view_board.make_move_unchecked(mv);
                    }
                }
            }
            
        }
        // Render history first as render_board can modify history part way through a frame
        self.render_history(ctx);
        self.render_board(ctx);
        self.render_game_over(ctx);
    }
    fn separator_drag(
        &mut self,
        ui: &mut egui::Ui,
        drag_axis: egui::Vec2,
        total_size: f32,
    ) {
        let sep_response = ui.separator();
        let drag_rect = sep_response.rect.expand(4.0);
        let id = sep_response.id;
        let drag_response = ui.interact(drag_rect, id, egui::Sense::drag());

        if drag_response.hovered() || drag_response.dragged() {
            ui.ctx().request_repaint();
        }

        let thin_rect = sep_response.rect;
        let color = if drag_response.dragged() {
            ui.visuals().widgets.active.bg_stroke.color
        } else if drag_response.hovered() {
            ui.visuals().widgets.hovered.bg_stroke.color
        } else {
            ui.visuals().widgets.inactive.bg_stroke.color
        };

        if drag_axis.x > 0.0 {
            // vertical line
            ui.painter().line_segment(
                [thin_rect.center_top(), thin_rect.center_bottom()],
                egui::Stroke::new(1.0, color),
            );
        } else {
            // horizontal line
            ui.painter().line_segment(
                [thin_rect.left_center(), thin_rect.right_center()],
                egui::Stroke::new(1.0, color),
            );
        }

        if drag_response.dragged() {
            let delta = if drag_axis.x > 0.0 {
                drag_response.drag_delta().x
            } else {
                drag_response.drag_delta().y
            };
            self.split_ratio += delta / total_size;
            self.split_ratio = self.split_ratio.clamp(0.1, 0.9);
        }
    }

    fn render_moves_list(&mut self, ui: &mut egui::Ui, is_portrait: bool) {
        let button_size = egui::Vec2::new(80.0, 30.0);
        let spacing = 8.0;

        let moves = &self.board.history;
        let move_count = moves.len();

        let (rows, cols) = if is_portrait {
            let rows = ((ui.available_height() + spacing) / (button_size.y + spacing))
                .floor()
                .max(1.0) as usize;
            let cols = (move_count + rows - 1) / rows;
            (rows, cols)
        } else {
            let cols = ((ui.available_width() + spacing) / (button_size.x + spacing))
                .floor()
                .max(1.0) as usize;
            let rows = (move_count + cols - 1) / cols;
            (rows, cols)
        };

        for row in 0..rows {
            ui.horizontal(|ui| {
                for col in 0..cols {
                    let idx = if is_portrait {
                        col * rows + row
                    } else {
                        row * cols + col
                    };

                    if idx < move_count {
                        let mv = &moves[idx];
                        let is_current =
                            matches!(self.state, PlayState::Viewing(pos) if pos == idx + 1);

                        let button = egui::Button::new(format!("{}. {}", idx + 1, mv))
                            .min_size(button_size);

                        let button = if is_current {
                            button.fill(egui::Color32::GRAY)
                        } else {
                            button
                        };

                        if ui.add(button).clicked() {
                            self.state = PlayState::Viewing(idx + 1);
                        }

                        ui.add_space(spacing);
                    }
                }
            });

            ui.add_space(spacing);
        }
    }

    pub fn render_history(&mut self, ctx: &egui::Context) {
        let screen_size = ctx.screen_rect();
        let is_portrait = screen_size.height() > screen_size.width();

        if is_portrait {
            self.render_history_portrait(ctx);
        } else {
            self.render_history_landscape(ctx);
        }
    }

    fn render_history_portrait(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("move_history_bottom")
            .resizable(true)
            .min_height(50.0)
            .max_height(250.0)
            .show(ctx, |ui| {
                let separator_width = 6.0;
                let total_width = ui.available_width();
                let available_width = total_width - separator_width;

                let left_width = available_width * self.split_ratio;
                let right_width = available_width - left_width;

                ui.with_layout(
                    Layout::left_to_right(egui::Align::TOP), 
                    |ui| {

                        ui.vertical(|ui| {
                            ui.set_width(left_width);
                            ui.heading("Move History");

                            egui::ScrollArea::horizontal()
                                .id_salt("scroll_moves")
                                .auto_shrink([true, false])
                                .show(ui, |ui| {
                                    self.render_moves_list(ui, true);
                                });
                        });

                        self.separator_drag(ui, egui::vec2(1.0, 0.0), available_width);

                        ui.vertical(|ui| {
                            ui.set_width(right_width);
                            self.render_settings(ui);

                        });
                    });
            });
    }

    fn render_history_landscape(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("move_history_right")
            .resizable(true)
            .min_width(100.0)
            .max_width(300.0)
            .show(ctx, |ui| {
                let separator_height = 6.0;
                let total_height = ui.available_height().max(1.0);
                let available_height = total_height - separator_height;

                let top_height = available_height * self.split_ratio;
                let bot_height = available_height - top_height;

                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), top_height),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        ui.heading("Move History");
                        egui::ScrollArea::vertical()
                            .id_salt("scroll_moves")
                            .auto_shrink([false, true])
                            .show(ui, |ui| {
                                self.render_moves_list(ui, false);
                            });
                    },
                );

                self.separator_drag(ui, egui::vec2(0.0, 1.0), available_height);

                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), bot_height),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        self.render_settings(ui);
                    },
                );
            });
    }
    pub fn render_settings(&mut self, ui: &mut Ui) {
        ui.heading("Settings");

        ui.add_space(20.0);
        
        egui::ScrollArea::vertical()
            .id_salt("scroll_settings")
            .auto_shrink([false, true])
            .show(ui, |ui| {
                self.render_engine_side_selector(ui);
                ui.add_space(8.0);
                ui.checkbox(&mut self.auto_queen, "Auto-queen:");
                ui.add_space(8.0);
                ui.label("Seconds/Move");
                let slider = Slider::new(&mut self.seconds_per_move, RangeInclusive::new(0.1, 5.0));
                ui.add(slider);
        });
    }
    pub fn render_board(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
                    egui::Sense::click(),
                );
                
                let origin = response.rect.min;
                let board = if let PlayState::Viewing(_) = self.state { &self.view_board } else { &self.board };

                self.render_tiles(&painter, origin, board);
                self.render_pieces(&painter, origin, board);
                self.render_moves(&painter, origin, board);

                self.handle_play_state(ui, ctx, response, origin);
                


                if horizontal_margin > 0.0 {
                    ui.add_space(horizontal_margin);
                }
            });

            ui.add_space(12.0);

            ui.vertical_centered(|ui| {
                let font_size = self.board_size * 0.02;
                let board = if let PlayState::Viewing(_) = self.state { &self.view_board } else { &self.board };
                let fen_text = RichText::new(format!("FEN: {}", board.to_fen()))
                    .monospace()
                    .size(font_size);
                ui.label(fen_text);

                ui.add_space(8.0);
            });
        });
    }
    
}