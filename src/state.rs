use ggez::
{
    Context,
    GameResult,
};

pub enum State
{
    Void,
}

pub enum Transition
{
    Push(State),
    Pop,
}

impl State
{
    pub fn update(&self, ctx: &mut Context) -> GameResult<Option<Transition>>
    {
        match self
        {
            _ => Ok(None)
        }
    }

    pub fn draw(&self, ctx: &mut Context, scale: f32, offset: [f32; 2])
        -> GameResult
    {
        match self
        {
            _ => Ok(())
        }
    }

    pub fn size(&self) -> [u32; 2]
    {
        match self
        {
            _ => [0; 2],
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
