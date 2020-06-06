use ggez::
{
    Context,
    GameResult,
    graphics::Mesh,
};

use super::shape::{Shape, ShapeData, NUM_SHAPES};

pub struct ShapeQueue
{
    index: usize,

    bag_one: [Shape; NUM_SHAPES as usize],
    bag_two: [Shape; NUM_SHAPES as usize],

    first: Mesh,
    second: Mesh,
    third: Mesh,

    outline: Mesh,
}

impl ShapeQueue
{
    pub fn new(ctx: &mut Context, shape_data: &ShapeData)
        -> GameResult<ShapeQueue>
    {
        use ggez::graphics::{Rect, DrawMode, MeshBuilder};
        use super::BOARD_DIMENSIONS;
        use super::rotation::Rotation;

        let bag_one = Self::generate_bag();
        let bag_two = Self::generate_bag();

        let first = shape_data.get_mesh(
            bag_one[0], Rotation::Zero, false).clone();

        let second = shape_data.get_mesh(
            bag_one[1], Rotation::Zero, false).clone();

        let third = shape_data.get_mesh(
            bag_one[2], Rotation::Zero, false).clone();

        let sixteenth = 1.0 / 16.0;

        let mut outline = MeshBuilder::new();

        // top outline
        outline.rectangle(
            DrawMode::fill(),
            Rect::new(
                1.0 + BOARD_DIMENSIONS[0] as f32 + 1.0 - sixteenth,
                1.0 + 3.0 + 1.0 - sixteenth,
                2.0 + 2.0 * sixteenth,
                sixteenth),
            (255, 255, 255).into());

        // bottom outline
        outline.rectangle(
            DrawMode::fill(),
            Rect::new(
                1.0 + BOARD_DIMENSIONS[0] as f32 + 1.0 - sixteenth,
                1.0 + 3.0 + 1.0 + 6.0,
                2.0 + 2.0 * sixteenth,
                sixteenth),
            (255, 255, 255).into());

        // left outline
        outline.rectangle(
            DrawMode::fill(),
            Rect::new(
                1.0 + BOARD_DIMENSIONS[0] as f32 + 1.0 - sixteenth,
                1.0 + 3.0 + 1.0,
                sixteenth,
                6.0),
            (255, 255, 255).into());

        // right outline
        outline.rectangle(
            DrawMode::fill(),
            Rect::new(
                1.0 + BOARD_DIMENSIONS[0] as f32 + 1.0 + 2.0,
                1.0 + 3.0 + 1.0,
                sixteenth,
                6.0),
            (255, 255, 255).into());

        let outline = outline.build(ctx)?;

        Ok(ShapeQueue
        {
            index: 0,

            bag_one: bag_one,
            bag_two: bag_two,

            first: first,
            second: second,
            third: third,

            outline: outline,
        })
    }

    pub fn next(&mut self, shape_data: &ShapeData) -> Shape
    {
        use super::rotation::Rotation;

        let top = self.bag_one[self.index];

        self.index += 1;

        if self.index >= NUM_SHAPES as usize
        {
            self.index = 0;
            self.bag_one = self.bag_two;
            self.bag_two = Self::generate_bag();
        }

        let get_shape = &|i| if i < NUM_SHAPES as usize
        {
            self.bag_one[i]
        }
        else
        {
            self.bag_two[i - NUM_SHAPES as usize]
        };

        let first = shape_data.get_mesh(
            get_shape(self.index), Rotation::Zero, false).clone();
        let second = shape_data.get_mesh(
            get_shape(self.index + 1), Rotation::Zero, false).clone();
        let third = shape_data.get_mesh(
            get_shape(self.index + 2), Rotation::Zero, false).clone();

        self.first = first;
        self.second = second;
        self.third = third;

        top
    }

    fn generate_bag() -> [Shape; NUM_SHAPES as usize]
    {
        use rand::{thread_rng, seq::SliceRandom};

        let mut bag = [
            Shape::F,
            Shape::I,
            Shape::L,
            Shape::N,
            Shape::P,
            Shape::T,
            Shape::U,
            Shape::V,
            Shape::W,
            Shape::X,
            Shape::Y,
            Shape::Z,
        ];

        let mut rng = thread_rng();

        bag.shuffle(&mut rng);

        bag
    }

    pub fn draw(&self, ctx: &mut Context, scale: f32, offset: [f32; 2])
        -> GameResult
    {
        use ggez::graphics::{DrawParam, draw};

        let start = [
            1.0 + super::BOARD_DIMENSIONS[0] as f32 + 1.0,
            1.0 + 3.0 + 1.0,
        ];

        draw(ctx, &self.first, DrawParam::default()
            .dest([
                offset[0] + start[0] * scale,
                offset[1] + start[1] * scale,
            ])
            .scale([
                scale * (2.0 / 5.0),
                scale * (2.0 / 5.0),
            ]))?;

        draw(ctx, &self.second, DrawParam::default()
            .dest([
                offset[0] + start[0] * scale,
                offset[1] + (start[1] + 2.0) * scale,
            ])
            .scale([
                scale * (2.0 / 5.0),
                scale * (2.0 / 5.0),
            ]))?;

        draw(ctx, &self.third, DrawParam::default()
            .dest([
                offset[0] + start[0] * scale,
                offset[1] + (start[1] + 4.0) * scale,
            ])
            .scale([
                scale * (2.0 / 5.0),
                scale * (2.0 / 5.0),
            ]))?;

        draw(ctx, &self.outline, DrawParam::default()
            .dest(offset)
            .scale([scale, scale]))?;

        Ok(())
    }
}
