use chess_lib::Board;

pub struct PositionTab
{
    board: Board,
    pub should_close: bool,
}

impl PositionTab
{
    pub fn new() -> Self {
        Self { 
            board: Board::new(), 
            should_close: false,
        }
    }
}