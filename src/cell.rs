#[derive(Copy, Clone, Debug)]
pub enum CellState {
    Empty = 0,
    Populated = 1,
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub state: CellState,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            state: CellState::Empty,
        }
    }

    pub fn flip(&mut self) {
        self.state = match self.state {
            CellState::Empty => CellState::Populated,
            CellState::Populated => CellState::Empty,
        };
    }
}