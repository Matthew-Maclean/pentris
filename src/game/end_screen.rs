use ggez::
{
    Context,
    GameResult,
    graphics::Mesh,
};

pub struct EndScreen
{
    y: Mesh,
    o: Mesh,
    u: Mesh,
    l: Mesh,
    s: Mesh,
    e: Mesh,
    open_square: Mesh,
    r: Mesh,
    close_square: Mesh,
}

impl EndScreen
{
    pub fn new(ctx: &mut Context) -> GameResult<EndScreen>
    {
        use ggez::graphics::
        {
            Rect,
            DrawMode,
            MeshBuilder,
        };

        let eighth = 1.0 / 8.0;

        let y = MeshBuilder::new()
            // {
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth,
                    eighth,
                    eighth * 3.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth,
                    eighth * 3.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 3.0,
                    eighth * 4.0,
                    eighth * 2.0,
                    eighth * 3.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 5.0,
                    eighth * 2.0,
                    eighth,
                    eighth * 3.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 6.0,
                    eighth,
                    eighth,
                    eighth * 3.0),
                (255, 255, 255).into())
            .build(ctx)?; // }

        let o = MeshBuilder::new()
            // {
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 2.0,
                    eighth,
                    eighth * 4.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth * 4.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 2.0,
                    eighth * 6.0,
                    eighth * 4.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 5.0,
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth * 4.0),
                (255, 255, 255).into())
            .build(ctx)?; // }

        let u = MeshBuilder::new()
            // {
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth,
                    eighth * 2.0,
                    eighth * 5.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 2.0,
                    eighth * 6.0,
                    eighth * 4.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 5.0,
                    eighth,
                    eighth * 2.0,
                    eighth * 5.0),
                (255, 255, 255).into())
            .build(ctx)?; // }

        let l = MeshBuilder::new()
            // {
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth,
                    eighth * 2.0,
                    eighth * 6.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 3.0,
                    eighth * 6.0,
                    eighth * 4.0,
                    eighth),
                (255, 255, 255).into())
            .build(ctx)?; // }

        let s = MeshBuilder::new()
            // {
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 2.0,
                    eighth,
                    eighth * 5.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 2.0,
                    eighth * 3.0,
                    eighth * 4.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 5.0,
                    eighth * 4.0,
                    eighth * 2.0,
                    eighth * 2.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth * 6.0,
                    eighth * 5.0,
                    eighth),
                (255, 255, 255).into())
            .build(ctx)?; // }

        let e = MeshBuilder::new()
            // {
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth,
                    eighth * 2.0,
                    eighth * 6.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 3.0,
                    eighth,
                    eighth * 4.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 3.0,
                    eighth * 3.0,
                    eighth * 3.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 3.0,
                    eighth * 6.0,
                    eighth * 4.0,
                    eighth),
                (255, 255, 255).into())
            .build(ctx)?; // }

        let open_square = MeshBuilder::new()
            // {
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 4.0,
                    eighth,
                    eighth * 3.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 4.0,
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth * 4.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 4.0,
                    eighth * 6.0,
                    eighth * 3.0,
                    eighth),
                (255, 255, 255).into())
            .build(ctx)?; // }

        let r = MeshBuilder::new()
            // {
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth,
                    eighth * 5.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth * 5.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 3.0,
                    eighth * 4.0,
                    eighth * 3.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 5.0,
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth * 2.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 4.0,
                    eighth * 5.0,
                    eighth * 3.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 5.0,
                    eighth * 5.0,
                    eighth * 2.0,
                    eighth * 2.0),
                (255, 255, 255).into())
            .build(ctx)?; // }

        let close_square = MeshBuilder::new()
            // {
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth,
                    eighth * 3.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth * 4.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth * 6.0,
                    eighth * 3.0,
                    eighth),
                (255, 255, 255).into())
            .build(ctx)?; // }

        Ok(EndScreen
        {
            y: y,
            o: o,
            u: u,
            l: l,
            s: s,
            e: e,
            open_square: open_square,
            r: r,
            close_square: close_square,
        })
    }

    pub fn draw(&self, ctx: &mut Context, scale: f32, offset: [f32; 2])
        -> GameResult
    {
        use ggez::graphics::
        {
            DrawParam,
            draw,
        };

        let start = [
            (1.0 + (super::BOARD_DIMENSIONS[0] / 2) as f32 - 2.0) * scale,
            (1.0 + (super::BOARD_DIMENSIONS[1] / 4) as f32) * scale,
        ];

        // YOU
        draw(ctx, &self.y, DrawParam::default()
            .dest([
                offset[0] + start[0] + 1.0 * scale,
                offset[1] + start[1],
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.o, DrawParam::default()
            .dest([
                offset[0] + start[0] + 2.0 * scale,
                offset[1] + start[1],
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.u, DrawParam::default()
            .dest([
                offset[0] + start[0] + 3.0 * scale,
                offset[1] + start[1],
            ])
            .scale([scale, scale]))?;

        // LOSE
        draw(ctx, &self.l, DrawParam::default()
            .dest([
                offset[0] + start[0] + 0.5 * scale,
                offset[1] + start[1] + 1.0 * scale,
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.o, DrawParam::default()
            .dest([
                offset[0] + start[0] + 1.5 * scale,
                offset[1] + start[1] + 1.0 * scale,
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.s, DrawParam::default()
            .dest([
                offset[0] + start[0] + 2.5 * scale,
                offset[1] + start[1] + 1.0 * scale,
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.e, DrawParam::default()
            .dest([
                offset[0] + start[0] + 3.5 * scale,
                offset[1] + start[1] + 1.0 * scale,
            ])
            .scale([scale, scale]))?;

        // [R]
        draw(ctx, &self.open_square, DrawParam::default()
            .dest([
                offset[0] + start[0] + 1.0 * scale,
                offset[1] + start[1] + 2.0 * scale,
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.r, DrawParam::default()
            .dest([
                offset[0] + start[0] + 2.0 * scale,
                offset[1] + start[1] + 2.0 * scale,
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.close_square, DrawParam::default()
            .dest([
                offset[0] + start[0] + 3.0 * scale,
                offset[1] + start[1] + 2.0 * scale,
            ])
            .scale([scale, scale]))?;

        Ok(())
    }
}
