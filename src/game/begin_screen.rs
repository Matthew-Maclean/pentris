use ggez::
{
    Context,
    GameResult,
    graphics::Mesh,
};

pub struct BeginScreen
{
    p: Mesh,
    r: Mesh,
    e: Mesh,
    s: Mesh,
    open_square: Mesh,
    n: Mesh,
    t: Mesh,
    close_square: Mesh,
    o: Mesh,
    b: Mesh,
    g: Mesh,
    i: Mesh,
}

impl BeginScreen
{
    pub fn new(ctx: &mut Context) -> GameResult<BeginScreen>
    {
        use ggez::graphics::
        {
            Rect,
            DrawMode,
            MeshBuilder,
        };

        let eighth = 1.0 / 8.0;

        let p = MeshBuilder::new()
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
                    eighth * 5.0,
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth * 2.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 3.0,
                    eighth * 4.0,
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

        let n = MeshBuilder::new()
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
                    eighth * 2.0,
                    eighth,
                    eighth * 2.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 4.0,
                    eighth * 3.0,
                    eighth,
                    eighth * 2.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 5.0,
                    eighth,
                    eighth * 2.0,
                    eighth * 6.0),
                (255, 255, 255).into())
            .build(ctx)?; // }

        let t = MeshBuilder::new()
            // {
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth,
                    eighth * 6.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 3.0,
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth * 5.0),
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

        let b = MeshBuilder::new()
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
                    eighth * 3.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 5.0,
                    eighth * 2.0,
                    eighth * 2.0,
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
                    eighth * 5.0,
                    eighth * 4.0,
                    eighth * 2.0,
                    eighth * 2.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 3.0,
                    eighth * 6.0,
                    eighth * 3.0,
                    eighth),
                (255, 255, 255).into())
            .build(ctx)?; // }

        let g = MeshBuilder::new()
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
                    eighth * 4.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 4.0,
                    eighth * 3.0,
                    eighth * 2.0,
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
                    eighth * 2.0,
                    eighth * 6.0,
                    eighth * 4.0,
                    eighth),
                (255, 255, 255).into())
            .build(ctx)?; // }

        let i = MeshBuilder::new()
            // {
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth,
                    eighth * 6.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 3.0,
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth * 4.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth * 6.0,
                    eighth * 6.0,
                    eighth),
                (255, 255, 255).into())
            .build(ctx)?; // }

        Ok(BeginScreen
        {
            p: p,
            r: r,
            e: e,
            s: s,
            open_square: open_square,
            n: n,
            t: t,
            close_square: close_square,
            o: o,
            b: b,
            g: g,
            i: i,
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
            (1.0 + (super::BOARD_DIMENSIONS[0] / 2) as f32 - 3.0) * scale,
            (1.0 + (super::BOARD_DIMENSIONS[1] / 4) as f32) * scale,
        ];

        // PRESS
        draw(ctx, &self.p, DrawParam::default()
            .dest([
                offset[0] + start[0] + 1.0 * scale,
                offset[1] + start[1],
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.r, DrawParam::default()
            .dest([
                offset[0] + start[0] + 2.0 * scale,
                offset[1] + start[1],
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.e, DrawParam::default()
            .dest([
                offset[0] + start[0] + 3.0 * scale,
                offset[1] + start[1],
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.s, DrawParam::default()
            .dest([
                offset[0] + start[0] + 4.0 * scale,
                offset[1] + start[1],
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.s, DrawParam::default()
            .dest([
                offset[0] + start[0] + 5.0 * scale,
                offset[1] + start[1],
            ])
            .scale([scale, scale]))?;

        // [ENTER]
        draw(ctx, &self.open_square, DrawParam::default()
            .dest([
                offset[0] + start[0],
                offset[1] + start[1] + 1.0 * scale,
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.e, DrawParam::default()
            .dest([
                offset[0] + start[0] + 1.0 * scale,
                offset[1] + start[1] + 1.0 * scale,
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.n, DrawParam::default()
            .dest([
                offset[0] + start[0] + 2.0 * scale,
                offset[1] + start[1] + 1.0 * scale,
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.t, DrawParam::default()
            .dest([
                offset[0] + start[0] + 3.0 * scale,
                offset[1] + start[1] + 1.0 * scale,
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.e, DrawParam::default()
            .dest([
                offset[0] + start[0] + 4.0 * scale,
                offset[1] + start[1] + 1.0 * scale,
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.r, DrawParam::default()
            .dest([
                offset[0] + start[0] + 5.0 * scale,
                offset[1] + start[1] + 1.0 * scale,
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.close_square, DrawParam::default()
            .dest([
                offset[0] + start[0] + 6.0 * scale,
                offset[1] + start[1] + 1.0 * scale,
            ])
            .scale([scale, scale]))?;

        // TO
        draw(ctx, &self.t, DrawParam::default()
            .dest([
                offset[0] + start[0] + 2.5 * scale,
                offset[1] + start[1] + 2.0 * scale,
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.o, DrawParam::default()
            .dest([
                offset[0] + start[0] + 3.5 * scale,
                offset[1] + start[1] + 2.0 * scale,
            ])
            .scale([scale, scale]))?;

        // BEGIN
        draw(ctx, &self.b, DrawParam::default()
            .dest([
                offset[0] + start[0] + 1.0 * scale,
                offset[1] + start[1] + 3.0 * scale,
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.e, DrawParam::default()
            .dest([
                offset[0] + start[0] + 2.0 * scale,
                offset[1] + start[1] + 3.0 * scale,
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.g, DrawParam::default()
            .dest([
                offset[0] + start[0] + 3.0 * scale,
                offset[1] + start[1] + 3.0 * scale,
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.i, DrawParam::default()
            .dest([
                offset[0] + start[0] + 4.0 * scale,
                offset[1] + start[1] + 3.0 * scale,
            ])
            .scale([scale, scale]))?;
        draw(ctx, &self.n, DrawParam::default()
            .dest([
                offset[0] + start[0] + 5.0 * scale,
                offset[1] + start[1] + 3.0 * scale,
            ])
            .scale([scale, scale]))?;

        Ok(())
    }
}
