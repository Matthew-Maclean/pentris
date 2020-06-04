use ggez::
{
    Context,
    GameResult,
    event::EventHandler,
};

pub struct Window
{

}

impl Window
{
    pub fn new(_ctx: &mut Context) -> GameResult<Window>
    {
        Ok(Window
        {
        })
    }
}

impl EventHandler for Window
{
    fn update(&mut self, _ctx: &mut Context) -> GameResult
    {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult
    {
        use ggez::graphics::
        {
            clear,
            present,
        };

        clear(ctx, (0, 0, 0).into());

        present(ctx)
    }
}
