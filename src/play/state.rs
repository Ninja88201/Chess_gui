use chess_lib::{GameState, Tile};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlayState
{
    Playing(GameState),
    Viewing(usize),
    Promotion(Tile),
}