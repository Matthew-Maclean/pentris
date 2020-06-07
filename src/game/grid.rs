use ggez::
{
    Context,
    GameResult,
    graphics::
    {
        Mesh,
        Image,
        DrawParam,
        spritebatch::
        {
            SpriteBatch,
            SpriteIdx,
        },
    },
};

use super::shape::Tiles;

const FILLED_COLOUR: (u8, u8, u8) = (128, 128, 128);

pub struct Grid
{
    tiles:
    [
        [
            bool;
            super::BOARD_DIMENSIONS[1] as usize
        ];
        super::BOARD_DIMENSIONS[0] as usize
    ],
    indices:
    [
        [
            SpriteIdx;
            super::BOARD_DIMENSIONS[1] as usize
        ];
        super::BOARD_DIMENSIONS[0] as usize
    ],
    batch: SpriteBatch,
    outline: Mesh,
}

impl Grid
{
    pub fn new(ctx: &mut Context) -> GameResult<Grid>
    {
        let tiles = [
            [false; super::BOARD_DIMENSIONS[1] as usize];
            super::BOARD_DIMENSIONS[0] as usize
        ];

        let mut batch = SpriteBatch::new(
            Image::solid(ctx, 1, FILLED_COLOUR.into())?);

        // do this to avoid annoying unsafe code
        let token = batch.add(DrawParam::default()
            .dest([10_000.0, 10_000.0])
            .scale([0.0, 0.0]));

        let mut indices = [
            [token; super::BOARD_DIMENSIONS[1] as usize];
            super::BOARD_DIMENSIONS[0] as usize
        ];

        for x in 0..super::BOARD_DIMENSIONS[0] as usize
        {
            for y in 0..super::BOARD_DIMENSIONS[1] as usize
            {
                indices[x][y] = batch.add(DrawParam::default()
                    .scale([0.0, 0.0])
                    .dest([
                        x as f32,
                        y as f32,
                    ]));
            }
        }

        let quarter = 1.0 / 4.0;
        let sixteenth = 1.0 / 16.0;
        let thirtysecond = 1.0 / 32.0;

        let mut outline = ggez::graphics::MeshBuilder::new();

        use ggez::graphics::{Rect, DrawMode};

        // top outline
        outline.rectangle(
            DrawMode::fill(),
            Rect::new(
                1.0 - sixteenth,
                1.0 - sixteenth,
                super::BOARD_DIMENSIONS[0] as f32 + 2.0 * sixteenth,
                sixteenth),
            (255, 255, 255).into());

        // left outline
        outline.rectangle(
            DrawMode::fill(),
            Rect::new(
                1.0 - sixteenth,
                1.0,
                sixteenth,
                super::BOARD_DIMENSIONS[1] as f32),
            (255, 255, 255).into());

        // bottom outline
        outline.rectangle(
            DrawMode::fill(),
            Rect::new(
                1.0 - sixteenth,
                1.0 + super::BOARD_DIMENSIONS[1] as f32,
                super::BOARD_DIMENSIONS[0] as f32 + 2.0 * sixteenth,
                sixteenth),
            (255, 255, 255).into());

        // right outline
        outline.rectangle(
            DrawMode::fill(),
            Rect::new(
                1.0 + super::BOARD_DIMENSIONS[0] as f32,
                1.0,
                sixteenth,
                super::BOARD_DIMENSIONS[1] as f32),
            (255, 255, 255).into());

        // bottom ticks
        for x in 0..(super::BOARD_DIMENSIONS[0] - 1)
        {
            outline.rectangle(
                DrawMode::fill(),
                Rect::new(
                    1.0 + (x + 1) as f32 - thirtysecond,
                    1.0 + super::BOARD_DIMENSIONS[1] as f32 - quarter,
                    sixteenth,
                    quarter),
                (255, 255, 255).into());
        }

        // left ticks
        for y in 0..(super::BOARD_DIMENSIONS[1] - 1)
        {
            outline.rectangle(
                DrawMode::fill(),
                Rect::new(
                    1.0,
                    1.0 + (y + 1) as f32 - thirtysecond,
                    quarter,
                    sixteenth),
                (255, 255, 255).into());
        }

        let outline = outline.build(ctx)?;

        Ok(Grid
        {
            tiles: tiles,
            indices: indices,
            batch: batch,
            outline: outline,
        })
    }

    pub fn set_tiles(&mut self, tiles: &Tiles, pos: [i32; 2]) -> GameResult
    {
        for tile in super::shape::stamp_tiles(tiles, pos).iter()
        {
            self.fill_tile(*tile)?;
        }

        Ok(())
    }

    fn fill_tile(&mut self, tile: [i32; 2]) -> GameResult
    {
        if Grid::in_bounds(tile)
        {
            let y = (super::BOARD_DIMENSIONS[1] - 1 - tile[1] as u32) as usize;
            let x = tile[0] as usize;

            self.tiles[x][y] = true;
            self.batch.set(self.indices[x][y], DrawParam::default()
                .dest([x as f32, y as f32])
                .scale([1.0, 1.0]))?;
        }

        Ok(())
    }

    pub fn check_line(&self, line: usize) -> bool
    {
        let mut row = [false; super::BOARD_DIMENSIONS[0] as usize];

        for x in 0..super::BOARD_DIMENSIONS[0] as usize
        {
            row[x] = self.tiles[x][line];
        }

        row == [true; super::BOARD_DIMENSIONS[0] as usize]
    }

    pub fn clear_line(&mut self, line: usize) -> GameResult
    {
        if line < super::BOARD_DIMENSIONS[1] as usize
        {
            for x in 0..super::BOARD_DIMENSIONS[0] as usize
            {
                self.tiles[x][line] = false;

                self.batch.set(self.indices[x][line], DrawParam::default()
                    .scale([0.0, 0.0]))?;
            }
        }

        Ok(())
    }

    pub fn move_down(
        &mut self, clear: [bool; super::BOARD_DIMENSIONS[1] as usize])
        -> GameResult
    {
        for y in 0..super::BOARD_DIMENSIONS[1] as usize
        {
            if clear[y]
            {
                for y in (0..=y).rev()
                {
                    for x in 0..super::BOARD_DIMENSIONS[0] as usize
                    {
                        if y == 0
                        {
                            self.tiles[x][y] = false;
                        }
                        else
                        {
                            self.tiles[x][y] = self.tiles[x][y - 1];
                        }
                    }
                }
            }
        }

        for y in 0..super::BOARD_DIMENSIONS[1] as usize
        {
            for x in 0..super::BOARD_DIMENSIONS[0] as usize
            {
                if self.tiles[x][y]
                {
                    self.batch.set(self.indices[x][y], DrawParam::default()
                        .dest([x as f32, y as f32])
                        .scale([1.0, 1.0]))?;
                }
                else
                {
                    self.batch.set(self.indices[x][y], DrawParam::default()
                        .dest([x as f32, y as f32])
                        .scale([0.0, 0.0]))?;
                }
            }
        }

        Ok(())
    }

    pub fn clear(&mut self) -> GameResult
    {
        for y in 0..super::BOARD_DIMENSIONS[1]
        {
            for x in 0..super::BOARD_DIMENSIONS[0]
            {
                let y = (super::BOARD_DIMENSIONS[1] - 1 - y) as usize;
                let x = x as usize;

                self.tiles[x][y] = false;
                self.batch.set(self.indices[x][y], DrawParam::default()
                    .dest([x as f32, y as f32])
                    .scale([0.0, 0.0]))?;
            }
        }

        Ok(())
    }

    pub fn is_set(&self, pos: [i32; 2]) -> bool
    {
        let pos = [pos[0], (super::BOARD_DIMENSIONS[1] as i32 - 1 - pos[1])];

        if Grid::in_bounds(pos)
        {
            self.tiles[pos[0] as usize][pos[1] as usize]
        }
        else
        {
            false
        }
    }

    pub fn draw(&self, ctx: &mut Context, scale: f32, offset: [f32; 2])
        -> GameResult
    {
        use ggez::graphics::draw;

        draw(ctx, &self.batch, DrawParam::default()
            .dest([
                offset[0] + 1.0 * scale,
                offset[1] + 1.0 * scale,
            ])
            .scale([scale; 2]))?;

        draw(ctx, &self.outline, DrawParam::default()
            .dest(offset)
            .scale([scale; 2]))?;

        Ok(())
    }

    pub fn in_bounds(pos: [i32; 2]) -> bool
    {
        pos[0] >= 0 && pos[0] < super::BOARD_DIMENSIONS[0] as i32 &&
            pos[1] >= 0 && pos[1] < super::BOARD_DIMENSIONS[1] as i32
    }
}
