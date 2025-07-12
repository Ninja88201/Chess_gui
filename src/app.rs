use chess_lib::Board;
use eframe::egui;
use egui::{Id, Modal};

use crate::{play::PlayTab, position_creator::PositionTab, utils::load_atlas};

pub enum Tab
{
    Play,
    PositionCreator,
    Statistics,
}
pub struct ChessApp {
    current_tab: Tab,

    play_tab: PlayTab,
    position_tab: PositionTab,

    show_modal: Option<String>,

}
impl ChessApp
{
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        let ctx = &cc.egui_ctx;

        Self {            
            current_tab: Tab::Play,
            play_tab: 
                PlayTab::new(
                    None,
                    // Some("2N1N3/1N3N2/8/1N3N2/2N1N3/8/8/k6K w - - 0 1"),
                    load_atlas(ctx),
                ),
                position_tab: PositionTab::new(
                    None,
                    load_atlas(ctx),
                ),
            show_modal: None,
        }
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.play_tab.should_close || self.position_tab.should_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if self.position_tab.change_tab {
            let position = self.position_tab.board.to_fen();
            match Board::new_from_fen(&position) {
                Ok(b) => {
                    self.play_tab.board = b.clone();
                    self.play_tab.view_board = b;
                    self.current_tab = Tab::Play;
                },
                Err(e) => {
                    self.show_modal = Some(e);
                },
                
            };
            self.position_tab.change_tab = false;
        }
        if let Some(s) = &self.show_modal {
            let text = s.clone();
            Modal::new(Id::new("modal"))
            .show(ctx, |ui| {
                ui.heading(text);

                if ui.button("Close").clicked() {
                    self.show_modal = None;
                }
            });
        }
        egui::TopBottomPanel::top("tabs_panel")
        .resizable(false)
        .min_height(50.0)
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 20.0;

                let mut style = (*ui.style_mut()).clone();
                style.override_text_style = Some(egui::TextStyle::Heading);
                ui.set_style(style);

                let tab_button = |ui: &mut egui::Ui, label: &str, selected: bool| {
                    ui.add_sized(
                        [150.0, 40.0], 
                        egui::SelectableLabel::new(selected, label),
                    )
                };

                if tab_button(ui, "Play", matches!(self.current_tab, Tab::Play)).clicked() {
                    self.current_tab = Tab::Play;
                }
                if tab_button(ui, "Position Creator", matches!(self.current_tab, Tab::PositionCreator)).clicked() {
                    self.current_tab = Tab::PositionCreator;
                }
                // if tab_button(ui, "Statistics", matches!(self.current_tab, Tab::Statistics)).clicked() {
                //     self.current_tab = Tab::Statistics;
                // }
            });
        });
        match self.current_tab {
            Tab::Play => self.play_tab.render(ctx),
            Tab::PositionCreator => self.position_tab.render(ctx),
            Tab::Statistics => (),
        }
    }
}
        