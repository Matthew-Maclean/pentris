use ggez::
{
    Context,
    GameResult,
    event::EventHandler,
    input::keyboard::{KeyCode, KeyMods},
};

use crate::game::Game;

pub struct Window
{
    size: [f32; 2],
    scale: f32,
    offset: [f32; 2],

    game: Game,
}

impl Window
{
    pub fn new(ctx: &mut Context) -> GameResult<Window>
    {
        let mut win = Window
        {
            size: [0.0; 2],
            scale: 0.0,
            offset: [0.0; 2],

            game: Game::new(ctx)?,
        };

        win.rescale(ctx);

        Ok(win)
    }

    fn rescale(&mut self, ctx: &mut Context)
    {
        self.size =
        {
            let s = ggez::graphics::drawable_size(ctx);
            [s.0, s.1]
        };

        let game_size = self.game.size();

        self.scale = f32::min(
            self.size[0] / game_size[0] as f32,
            self.size[1] / game_size[1] as f32);

        self.offset = [
            (self.size[0] - self.scale * game_size[0] as f32) / 2.0,
            (self.size[1] - self.scale * game_size[1] as f32) / 2.0];
    }
}

impl EventHandler for Window
{
    fn update(&mut self, ctx: &mut Context) -> GameResult
    {
        self.game.update(ctx)
    }

    fn key_down_event(
        &mut self, _ctx: &mut Context, key: KeyCode, _: KeyMods, repeat: bool)
    {
        self.game.key_down(key, repeat);
    }

    fn resize_event(&mut self, ctx: &mut Context, w: f32, h: f32)
    {
        ggez::graphics::set_screen_coordinates(ctx,
            ggez::graphics::Rect::new(0.0, 0.0, w, h)).unwrap();

        self.rescale(ctx);
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult
    {
        use ggez::graphics::
        {
            clear,
            present,
        };

        clear(ctx, (0, 0, 0).into());

        self.game.draw(ctx, self.scale, self.offset)?;

        present(ctx)
    }
}
