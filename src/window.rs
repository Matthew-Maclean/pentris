use ggez::
{
    Context,
    GameResult,
    event::EventHandler,
};

use crate::state::{State, Transition};

pub struct Window
{
    size: [f32; 2],
    scale: f32,
    offset: [f32; 2],

    states: Vec<State>,
}

impl Window
{
    pub fn new(ctx: &mut Context) -> GameResult<Window>
    {
        let size =
        {
            let s = ggez::graphics::drawable_size(ctx);
            [s.0, s.1]
        };

        let mut win = Window
        {
            size: size,
            scale: 0.0,
            offset: [0.0, 0.0],
            states: vec![State::default()],
        };

        win.rescale();

        Ok(win)
    }

    fn rescale(&mut self)
    {
        let state_size = self.states
            .last()
            .unwrap()
            .size();

        self.scale = f32::min(
            self.size[0] / state_size[0] as f32,
            self.size[1] / state_size[1] as f32);

        self.offset = [
            (self.size[0] - self.scale * state_size[0] as f32) / 2.0,
            (self.size[1] - self.scale * state_size[1] as f32) / 2.0];
    }
}

impl EventHandler for Window
{
    fn update(&mut self, ctx: &mut Context) -> GameResult
    {
        if let Some(tr) = self.states.last()
            .unwrap()
            .update(ctx)?
        {
            match tr
            {
                Transition::Push(state) => self.states.push(state),
                Transition::Pop =>
                {
                    self.states.pop();
                    if self.states.len() == 0
                    {
                        self.states.push(State::default());
                    }
                },
            }
        }

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

        self.states.last()
            .unwrap()
            .draw(ctx, self.scale, self.offset)?;

        present(ctx)
    }
}
