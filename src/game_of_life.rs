use super::Board;
use super::Canvas;
use stdweb::traits::IEventTarget;
use stdweb::web::event::ClickEvent;
use stdweb::web::event::IMouseEvent;

pub enum GameOfLifeStates {
    Menu,
    Paused,
    Playing,
}

pub struct GameOfLife {
    pub state: GameOfLifeStates,
    pub board: Board,
    canvas: Canvas,
}

impl GameOfLife {
    pub fn new(height: u32, width: u32, canvas_id: &str) -> Result<Self, ()> {
        let canvas = Canvas::new(canvas_id, height * 10, width * 10);

        if canvas.is_err() {
            stdweb::console!(error, format!("Could not find canvas {}", canvas_id));
            return Err(());
        }

        Ok(Self {
            state: GameOfLifeStates::Paused,
            board: Board::new(height, width),
            canvas: canvas.unwrap(),
        })
    }

    pub fn initialize(self) {
        self.canvas.fill("5BC2E7");

        self.canvas.get_elem().add_event_listener({
            |event: ClickEvent| {
                stdweb::console!(log, format!("{}, {}", event.offset_x(), event.offset_y()))
            }
        });
    }

    pub fn play(&mut self) {
        self.state = GameOfLifeStates::Playing;
    }
}