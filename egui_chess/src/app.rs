use eframe::egui;
use egui::{RichText, TextureHandle, Vec2};

mod render;
mod helper;
use helper::UIState;
mod input;

use chess_lib::{Board, Tile};
use rand::rngs::ThreadRng;

pub struct ChessApp {
    board: Board,
    rand: ThreadRng,
    flipped: bool,
    selected: Option<Tile>,

    atlas: TextureHandle,
    board_size: f32,

    ui_state: UIState,
    should_close: bool,
}
impl ChessApp
{
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;

        let texture = crate::utils::load_texture_from_png(ctx);
        Self {
            board: Board::new(),
            rand: rand::rng(),
            flipped: false,
            selected: None,
            atlas: texture,
            board_size: 800.0,
            ui_state: UIState::Playing,
            should_close: false,
        }
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.should_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        let screen_rect = ctx.screen_rect();
        let is_portrait = screen_rect.height() > screen_rect.width();

        if is_portrait {
            egui::TopBottomPanel::bottom("move_history_bottom")
                .min_height(100.0)
                .max_height(300.0)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.heading("Move History");

                    egui::ScrollArea::horizontal()
                        .auto_shrink([true, false])
                        .show(ui, |ui| {
                            let moves = &self.board.history;
                            let available_height = ui.available_height();

                            let button_height = 30.0;
                            let spacing = ui.spacing().item_spacing.y;

                            let rows = ((available_height + spacing) / (button_height + spacing))
                                .floor()
                                .max(1.0) as usize;

                            let cols = (moves.len() + rows - 1) / rows;

                            let column_width = 80.0;

                            let total_width = cols as f32 * (column_width + 8.0 /* spacing between cols */);
                            ui.set_min_width(total_width);

                            ui.horizontal(|ui| {
                                for col in 0..cols {
                                    ui.vertical(|ui| {
                                        ui.set_width(column_width);

                                        let total_height =
                                            button_height * rows as f32 + spacing * (rows.saturating_sub(1) as f32);
                                        ui.set_height(total_height);

                                        for row in 0..rows {
                                            let idx = col * rows + row;
                                            if idx < moves.len() {
                                                let mv = &moves[idx];
                                                let is_current =
                                                    matches!(self.ui_state, UIState::Viewing(pos) if pos == idx + 1);

                                                let button = egui::Button::new(format!("{}. {}", idx + 1, mv.to_string()))
                                                    .min_size(egui::Vec2::new(60.0, button_height));

                                                let button = if is_current {
                                                    button.fill(egui::Color32::GRAY)
                                                } else {
                                                    button
                                                };

                                                if ui.add(button).clicked() {
                                                    self.ui_state = UIState::Viewing(idx + 1);
                                                }
                                            }
                                        }
                                    });

                                    if col != cols - 1 {
                                        ui.add_space(4.0);
                                    }
                                }
                            });
                        });
                });
        } else {
            egui::SidePanel::right("move_history_right")
                .min_width(150.0)
                .max_width(400.0)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.heading("Move History");

                    let moves = &self.board.history;

                    let available_width = ui.available_width();

                    let button_width = 80.0;
                    let button_height = 30.0;
                    let spacing_x = ui.spacing().item_spacing.x;
                    let spacing_y = ui.spacing().item_spacing.y;

                    let cols = ((available_width + spacing_x) / (button_width + spacing_x)).floor().max(1.0) as usize;

                    let rows = (moves.len() + cols - 1) / cols;

                    egui::ScrollArea::vertical()
                        .auto_shrink([false, true])
                        .show(ui, |ui| {
                            for row in 0..rows {
                                ui.horizontal(|ui| {
                                    for col in 0..cols {
                                        let idx = row * cols + col;
                                        if idx < moves.len() {
                                            let mv = &moves[idx];
                                            let is_current = matches!(self.ui_state, UIState::Viewing(pos) if pos == idx + 1);

                                            let button = egui::Button::new(format!("{}. {}", idx + 1, mv.to_string()))
                                                .min_size(Vec2::new(button_width, button_height));

                                            let button = if is_current {
                                                button.fill(egui::Color32::GRAY)
                                            } else {
                                                button
                                            };

                                            if ui.add(button).clicked() {
                                                self.ui_state = UIState::Viewing(idx + 1);
                                            }

                                            ui.add_space(spacing_x);
                                        }
                                    }
                                });

                                ui.add_space(spacing_y);
                            }
                        });
                });
        }
        self.render_board(ctx);
    }
}
