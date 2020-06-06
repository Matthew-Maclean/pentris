use ggez::
{
    Context,
    GameResult,
    graphics::Mesh,
};

use super::{Shape, ShapeData};

pub struct StoreWindow
{
    stored: Option<(Shape, Mesh)>,

    outline: Mesh,
}

impl StoreWindow
{
    pub fn new(ctx: &mut Context) -> GameResult<StoreWindow>
    {
        use ggez::graphics::{Rect, DrawMode, MeshBuilder};
        use super::BOARD_DIMENSIONS;

        let sixteenth = 1.0 / 16.0;

        let mut outline = MeshBuilder::new();

        // top outline
        outline.rectangle(
            DrawMode::fill(),
            Rect::new(
                1.0 + BOARD_DIMENSIONS[0] as f32 + 1.0 - sixteenth,
                1.0 - sixteenth,
                3.0 + 2.0 * sixteenth,
                sixteenth),
            (255, 255, 255).into());

        // left outline
        outline.rectangle(
            DrawMode::fill(),
            Rect::new(
                1.0 + BOARD_DIMENSIONS[0] as f32 + 1.0 - sixteenth,
                1.0,
                sixteenth,
                3.0),
            (255, 255, 255).into());

        // right outline
        outline.rectangle(
            DrawMode::fill(),
            Rect::new(
                1.0 + BOARD_DIMENSIONS[0] as f32 + 1.0 + 3.0,
                1.0,
                sixteenth,
                3.0),
            (255, 255, 255).into());

        // bottom outline
        outline.rectangle(
            DrawMode::fill(),
            Rect::new(
                1.0 + BOARD_DIMENSIONS[0] as f32 + 1.0 - sixteenth,
                1.0 + 3.0,
                3.0 + 2.0 * sixteenth,
                sixteenth),
            (255, 255, 255).into());

        let outline = outline.build(ctx)?;

        Ok(StoreWindow
        {
            stored: None,

            outline: outline,
        })
    }

    pub fn store(&mut self, shape: Shape, data: &ShapeData) -> Option<Shape>
    {
        use super::Rotation;

        let prev = if let Some((shape, _)) = &self.stored
        {
            Some(*shape)
        }
        else
        {
            None
        };

        self.stored = Some((
            shape,
            data.get_mesh(shape, Rotation::Zero, false).clone()));

        prev
    }

    pub fn draw(&self, ctx: &mut Context, scale: f32, offset: [f32; 2])
        -> GameResult
    {
        use ggez::graphics::
        {
            DrawParam,
            draw,
        };

        if let Some((shape, ref mesh)) = self.stored
        {
            let sixteenth = 1.0 / 16.0;

            use super::BOARD_DIMENSIONS;

            draw(ctx, mesh, DrawParam::default()
                .dest([
                    offset[0] + (BOARD_DIMENSIONS[0] as f32 + 2.0 + sixteenth)
                        * scale,
                    offset[1] + (1.0 + sixteenth) * scale,
                ])
                .scale([
                    scale * ((3.0 - sixteenth * 2.0) / 5.0),
                    scale * ((3.0 - sixteenth * 2.0) / 5.0),
                ])
                .color(Shape::colour(shape)))?;
        }

        draw(ctx, &self.outline, DrawParam::default()
            .dest(offset)
            .scale([scale, scale]))?;

        Ok(())
    }
}
