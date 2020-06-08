use ggez::
{
    Context,
    GameResult,
    input::keyboard::KeyCode,
};

mod grid;
mod piece;
mod input;
mod shape;
mod rotation;
mod shape_queue;
mod store_window;

use grid::Grid;
use piece::Piece;
use input::Input;
use shape::{Shape, ShapeData};
use rotation::Rotation;
use shape_queue::ShapeQueue;
use store_window::StoreWindow;

const BOARD_DIMENSIONS: [u32; 2] = [15, 20];

pub struct Game
{
    phase: GamePhase,

    shape_data: ShapeData,

    inputs: Vec<Input>,

    grid: Grid,
    piece: Piece,
    store_window: StoreWindow,
    stored_recently: bool,
    shape_queue: ShapeQueue,

    score: u32,
    speed: u64,

    play_tick: u64,
    pause_tick: u64,
}

#[derive(Copy, Clone)]
pub enum GamePhase
{
    Begin,
    Normal,
    ClearPause([bool; BOARD_DIMENSIONS[1] as usize]),
}

impl Game
{
    pub fn new(ctx: &mut Context) -> GameResult<Game>
    {
        let shape_data = ShapeData::init(ctx)?;
        let mut shape_queue = ShapeQueue::new(ctx, &shape_data)?;
        let grid = Grid::new(ctx)?;
        let piece = Piece::new(
            shape_queue.next(&shape_data),
            [
                (BOARD_DIMENSIONS[0] / 2) as i32,
                (BOARD_DIMENSIONS[1] - 3) as i32
            ], &grid, &shape_data);
        Ok(Game
        {
            phase: GamePhase::Begin,

            shape_data: shape_data,

            inputs: Vec::new(),

            grid: grid,
            piece: piece,
            store_window: StoreWindow::new(ctx)?,
            stored_recently: false,
            shape_queue: shape_queue,

            score: 0,
            speed: 60,

            play_tick: 0,
            pause_tick: 0,
        })
    }

    pub fn update(&mut self) -> GameResult
    {
        match self.phase
        {
            GamePhase::Begin =>
            {
                while let Some(input) = self.inputs.pop()
                {
                    match input
                    {
                        Input::Select => self.phase = GamePhase::Normal,
                        _ => { },
                    }
                }
            }
            GamePhase::Normal => self.normal_tick()?,
            GamePhase::ClearPause(lines) =>
            {
                self.pause_tick += 1;

                if self.pause_tick % 20 == 0
                {
                    self.grid.move_down(lines)?;

                    self.pause_tick = 0;
                    self.phase = GamePhase::Normal;
                }
            },
        }

        Ok(())
    }

    fn normal_tick(&mut self) -> GameResult
    {
        self.play_tick += 1;

        let mut set = if self.play_tick % self.speed == 1
        {
            if !self.piece.shift([0, -1], &self.grid, &self.shape_data)
            {
                true
            }
            else
            {
                false
            }
        }
        else
        {
            false
        };

        while let Some(input) = self.inputs.pop()
        {
            match input
            {
                Input::Up => if !set && !self.stored_recently
                {
                    if let Some(stored) = self.store_window.stored()
                    {
                        let current = self.piece.shape();
                        if self.piece.set_shape(
                            stored, &self.grid, &self.shape_data)
                        {
                            self.store_window.store(current, &self.shape_data);
                            self.stored_recently = true;
                        }
                    }
                    else
                    {
                        let current = self.piece.shape();
                        if self.piece.set_shape(
                            self.shape_queue.peek(),
                            &self.grid,
                            &self.shape_data)
                        {
                            self.shape_queue.next(&self.shape_data);
                            self.store_window.store(current, &self.shape_data);
                            self.stored_recently = true;
                        }
                    }
                },
                Input::Down => if !set
                {
                    let pos = self.piece.shadow_pos(
                        &self.grid, &self.shape_data);

                    let diff = [
                        0,
                        pos[1] - self.piece.pos()[1],
                    ];

                    if diff == [0, 0]
                    {
                        set = true;
                    }
                    else
                    {
                        self.piece.shift(diff, &self.grid, &self.shape_data);
                    }
                },
                Input::Left => if !set
                {
                    self.piece.shift([-1, 0], &self.grid, &self.shape_data);
                },
                Input::Right => if !set
                {
                    self.piece.shift([1, 0], &self.grid, &self.shape_data);
                },
                Input::RotateCW => if !set
                {
                    self.piece.rotate_cw(&self.grid, &self.shape_data);
                },
                Input::RotateCCW => if !set
                {
                    self.piece.rotate_ccw(&self.grid, &self.shape_data);
                },
                Input::Flip => if !set
                {
                    self.piece.flip(&self.grid, &self.shape_data);
                },
                _ => { },
            }
        }

        if set
        {
            self.piece.set(&mut self.grid, &self.shape_data)?;

            self.piece = Piece::new(
                self.shape_queue.next(&self.shape_data),
                [
                    (BOARD_DIMENSIONS[0] / 2) as i32,
                    (BOARD_DIMENSIONS[1] - 3) as i32
                ],
                &self.grid,
                &self.shape_data);

            self.stored_recently = false;

            let mut lines = [false; BOARD_DIMENSIONS[1] as usize];
            let mut any_clear = false;

            for y in 0..BOARD_DIMENSIONS[1] as usize
            {
                lines[y] = self.grid.check_line(y);
                if lines[y]
                {
                    any_clear = true;
                    self.grid.clear_line(y)?;
                    self.score += 1;
                    if self.speed > 1
                    {
                        self.speed -= 1;
                    }
                }
            }

            if any_clear
            {
                self.phase = GamePhase::ClearPause(lines);
            }
        }

        Ok(())
    }

    pub fn key_down(&mut self, key: KeyCode, repeat: bool)
    {
        if !repeat
        {
            match key
            {
                KeyCode::W => self.inputs.push(Input::Up),
                KeyCode::S => self.inputs.push(Input::Down),
                KeyCode::A => self.inputs.push(Input::Left),
                KeyCode::D => self.inputs.push(Input::Right),
                KeyCode::Q => self.inputs.push(Input::RotateCCW),
                KeyCode::E => self.inputs.push(Input::RotateCW),
                KeyCode::LShift => self.inputs.push(Input::Flip),
                KeyCode::Return => self.inputs.push(Input::Select),
                _ => { },
            }
        }
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
