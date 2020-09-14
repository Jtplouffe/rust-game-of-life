use super::Board;

pub enum GameOfLifeStates {
    Paused,
    Playing,
}

pub struct GameOfLife {
    pub state: GameOfLifeStates,
    pub board: Board,
}

impl GameOfLife {
    pub fn new(height: u8, width: u8) -> Self {
        Self {
            state: GameOfLifeStates::Paused,
            board: Board::new(height, width),
        }
    }

    pub fn play(&mut self) {
        self.state = GameOfLifeStates::Playing;
    }
}