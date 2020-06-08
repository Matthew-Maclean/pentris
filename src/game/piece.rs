use ggez::
{
    Context,
    GameResult,
    graphics::Mesh,
};

use super::grid::Grid;
use super::shape::{Shape, ShapeData};
use super::rotation::Rotation;

pub struct Piece
{
    shape: Shape,
    rotation: Rotation,
    flip: bool,

    pos: [i32; 2],
    shadow: [i32; 2],

    mesh: Mesh,
}

impl Piece
{
    pub fn new(
        shape: Shape, pos: [i32; 2], grid: &Grid, shape_data: &ShapeData)
        -> Piece
    {
        let mut p = Piece
        {
            shape: shape,
            rotation: Rotation::Zero,
            flip: false,

            pos: pos,
            shadow: pos,

            mesh: shape_data.get_mesh(shape, Rotation::Zero, false).clone(),
        };

        p.shadow = p.shadow_pos(grid, shape_data);

        p
    }

    pub fn pos(&self) -> [i32; 2]
    {
        self.pos
    }

    pub fn shape(&self) -> Shape
    {
        self.shape
    }

    pub fn set_shape(
        &mut self, shape: Shape, grid: &Grid, shape_data: &ShapeData) -> bool
    {
        use super::shape::stamp_tiles;

        if stamp_tiles(
            shape_data
                .get_tiles(shape, self.rotation, self.flip),
                self.pos)
            .iter()
            .all(|tile| Grid::in_bounds_ex_top(*tile) && !grid.is_set(*tile))
        {
            self.shape = shape;
            self.mesh = shape_data.get_mesh(
                self.shape, self.rotation, self.flip).clone();
            self.shadow = self.shadow_pos(&grid, &shape_data);
            true
        }
        else
        {
            false
        }
    }

    pub fn shift(&mut self, dir: [i32; 2], grid: &Grid, shape_data: &ShapeData)
        -> bool
    {
        use super::shape::stamp_tiles;

        let new_pos = [self.pos[0] + dir[0], self.pos[1] + dir[1]];

        if stamp_tiles(
            shape_data
                .get_tiles(self.shape, self.rotation, self.flip),
                new_pos)
            .iter()
            .all(|tile| Grid::in_bounds_ex_top(*tile) && !grid.is_set(*tile))
        {
            self.pos = new_pos;
            self.shadow = self.shadow_pos(&grid, &shape_data);
            true
        }
        else
        {
            false
        }
    }

    pub fn rotate_cw(&mut self, grid: &Grid, shape_data: &ShapeData) -> bool
    {
        use super::shape::stamp_tiles;

        let new_rot = self.rotation.clockwise();

        if stamp_tiles(
            shape_data
                .get_tiles(self.shape, new_rot, self.flip),
                self.pos)
            .iter()
            .all(|tile| Grid::in_bounds_ex_top(*tile) && !grid.is_set(*tile))
        {
            self.rotation = new_rot;
            self.mesh = shape_data.get_mesh(
                self.shape, self.rotation, self.flip).clone();
            self.shadow = self.shadow_pos(grid, shape_data);
            true
        }
        else
        {
            false
        }
    }

    pub fn rotate_ccw(&mut self, grid: &Grid, shape_data: &ShapeData) -> bool
    {
        use super::shape::stamp_tiles;

        let new_rot = self.rotation.counterclockwise();

        if stamp_tiles(
            shape_data
                .get_tiles(self.shape, new_rot, self.flip),
                self.pos)
            .iter()
            .all(|tile| Grid::in_bounds_ex_top(*tile) && !grid.is_set(*tile))
        {
            self.rotation = new_rot;
            self.mesh = shape_data.get_mesh(
                self.shape, self.rotation, self.flip).clone();
            self.shadow = self.shadow_pos(grid, shape_data);
            true
        }
        else
        {
            false
        }
    }

    pub fn flip(&mut self, grid: &Grid, shape_data: &ShapeData) -> bool
    {
        use super::shape::stamp_tiles;

        let new_flip = !self.flip;

        if stamp_tiles(
            shape_data
                .get_tiles(self.shape, self.rotation, new_flip),
                self.pos)
            .iter()
            .all(|tile| Grid::in_bounds_ex_top(*tile) && !grid.is_set(*tile))
        {
            self.flip = new_flip;
            self.mesh = shape_data.get_mesh(
                self.shape, self.rotation, self.flip).clone();
            self.shadow = self.shadow_pos(grid, shape_data);
            true
        }
        else
        {
            false
        }
    }

    pub fn shadow_pos(&mut self, grid: &Grid, shape_data: &ShapeData)
        -> [i32; 2]
    {
        let mut pos = self.pos;

        while
        {
            let tiles = super::shape::stamp_tiles(
                shape_data.get_tiles(self.shape, self.rotation, self.flip),
                [pos[0], pos[1] - 1]);

            tiles.iter().all(|tile|
                Grid::in_bounds_ex_top(*tile) && !grid.is_set(*tile))
        }
        {
            pos[1] -= 1;
        }

        pos
    }

    pub fn set(&self, grid: &mut Grid, shape_data: &ShapeData) -> GameResult
    {
        grid.set_tiles(shape_data.get_tiles(
            self.shape, self.rotation, self.flip), self.pos)
    }

    pub fn draw(&self, ctx: &mut Context, scale: f32, offset: [f32; 2])
        -> GameResult
    {
        use ggez::graphics::
        {
            DrawParam,
            draw,
        };

        draw(ctx, &self.mesh, DrawParam::default()
            .dest([
                offset[0] + (1.0 + self.shadow[0] as f32 - 2.0) * scale,
                offset[1]
                    + (1.0
                    + super::BOARD_DIMENSIONS[1] as f32
                    - 1.0
                    - self.shadow[1] as f32
                    - 2.0) * scale,
            ])
            .scale([scale, scale])
            .color(self.shape.shadow_colour()))?;


        draw(ctx, &self.mesh, DrawParam::default()
            .dest([
                offset[0] + (1.0 + self.pos[0] as f32 - 2.0) * scale,
                offset[1]
                    + (1.0
                    + super::BOARD_DIMENSIONS[1] as f32
                    - 1.0
                    - self.pos[1] as f32
                    - 2.0) * scale,
            ])
            .scale([scale, scale])
            .color(self.shape.colour()))?;

        Ok(())
    }
}
