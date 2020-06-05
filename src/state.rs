use ggez::
{
    Context,
    GameResult,
};

use crate::game::Game;

pub enum State
{
    Void,
    Game(Game),
}

pub enum Transition
{
    Push(State),
    Pop,
}

impl State
{
    pub fn update(&mut self, ctx: &mut Context) -> GameResult<Option<Transition>>
    {
        match self
        {
            State::Game(ref mut game) => game.update(ctx),
            _ => Ok(None)
        }
    }

    pub fn draw(&self, ctx: &mut Context, scale: f32, offset: [f32; 2])
        -> GameResult
    {
        match self
        {
            State::Game(ref game) => game.draw(ctx, scale, offset),
            _ => Ok(())
        }
    }

    pub fn size(&self) -> [u32; 2]
    {
        match self
        {
            State::Game(ref game) => game.size(),
            _ => [1; 2], // don't divide by zero
        }
    }
}

impl Default for State
{
    fn default() -> Self
    {
        State::Void
    }
}
