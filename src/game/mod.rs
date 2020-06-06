use ggez::
{
    Context,
    GameResult,
    input::keyboard::KeyCode,
};

use crate::state::Transition;

mod grid;
mod piece;
mod shape;
mod rotation;
mod shape_queue;
mod store_window;

use grid::Grid;
use piece::Piece;
use shape::{Shape, ShapeData};
use rotation::Rotation;
use shape_queue::ShapeQueue;
use store_window::StoreWindow;

const BOARD_DIMENSIONS: [u32; 2] = [15, 20];

pub struct Game
{
    phase: GamePhase,

    shape_data: ShapeData,

    grid: Grid,
    piece: Piece,
    store_window: StoreWindow,
    shape_queue: ShapeQueue,
}

#[derive(Copy, Clone)]
pub enum GamePhase
{
    Begin,
    Normal,
}

impl Game
{
    pub fn new(ctx: &mut Context) -> GameResult<Game>
    {
        let shape_data = ShapeData::init(ctx)?;
        let mut shape_queue = ShapeQueue::new(ctx, &shape_data)?;
        let piece = Piece::new(
            shape_queue.next(&shape_data), [7, 9], &shape_data);
        Ok(Game
        {
            phase: GamePhase::Begin,

            shape_data: shape_data,

            grid: Grid::new(ctx)?,
            piece: piece,
            store_window: StoreWindow::new(ctx)?,
            shape_queue: shape_queue,
        })
    }

    pub fn update(&mut self, _ctx: &mut Context)
        -> GameResult<Option<Transition>>
    {
        Ok(None)
    }

    pub fn key_down(&mut self, key: KeyCode, repeat: bool)
    {
    }

    pub fn draw(&self, ctx: &mut Context, scale: f32, offset: [f32; 2])
        -> GameResult
    {
        self.piece.draw(ctx, scale, offset)?;

        self.grid.draw(ctx, scale, offset)?;

        self.store_window.draw(ctx, scale, offset)?;

        self.shape_queue.draw(ctx, scale, offset)?;

        Ok(())
    }

    pub fn size(&self) -> [u32; 2]
    {
        let width =
            1                   + // spacer
            BOARD_DIMENSIONS[0] + // board width
            1                   + // spacer
            3                   + // stored tile window
            1                   ; // spacer

        let height =
            1                   + // spacer
            BOARD_DIMENSIONS[1] + // board height
            1                   ; // spacer

        [width, height]
    }
}
