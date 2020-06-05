use ggez::
{
    Context,
    GameResult,
    ContextBuilder,
    conf::
    {
        WindowMode,
        WindowSetup,
    },
    event::run,
};

mod window;
mod state;
mod game;

use window::Window;

fn main() -> GameResult
{
    let (mut ctx, mut eloop) = ContextBuilder::new("pentris", "pentris")
        .window_setup(WindowSetup::default()
            .title("pentris"))
        .window_mode(WindowMode::default()
            .dimensions(500.0, 500.0)
            .resizable(true))
        .build()?;

    let mut window = Window::new(&mut ctx)?;

    run(&mut ctx, &mut eloop, &mut window)
}
