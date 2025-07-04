use chess_lib::{GameState, Tile};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlayState
{
    Playing(GameState),
    Viewing(usize),
    Promotion(Tile),
}
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Engine
{
    Neither,
    White,
    Black,
    Both,
}
impl Engine
{
    pub fn to_string(&self) -> &str
    {
        match self {
            Engine::Neither => "Neither",
            Engine::White => "White",
            Engine::Black => "Black",
            Engine::Both => "Both",
        }
    }
}