use ggez::
{
    Context,
    GameResult,
};

use crate::state::Transition;

mod grid;

use grid::Grid;

const BOARD_DIMENSIONS: [u32; 2] = [15, 20];

pub struct Game
{
    phase: GamePhase,

    grid: Grid,
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
        Ok(Game
        {
            phase: GamePhase::Begin,

            grid: Grid::new(ctx)?,
        })
    }

    pub fn update(&mut self, _ctx: &mut Context)
        -> GameResult<Option<Transition>>
    {
        Ok(None)
    }

    pub fn draw(&self, ctx: &mut Context, scale: f32, offset: [f32; 2])
        -> GameResult
    {
        self.grid.draw(ctx, scale, offset)?;

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
