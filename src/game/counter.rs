use ggez::
{
    Context,
    GameResult,
    graphics::Mesh,
};

pub struct Counter
{
    s: Mesh,
    c: Mesh,
    o: Mesh,
    r: Mesh,
    e: Mesh,

    p: Mesh,
    d: Mesh,

    score: u16,
    score_digits: [u8; 3],

    speed: u16,
    speed_digits: [u8; 3],

    zero: Mesh,
    one: Mesh,
    two: Mesh,
    three: Mesh,
    four: Mesh,
    five: Mesh,
    six: Mesh,
    seven: Mesh,
    eight: Mesh,
    nine: Mesh,
}

impl Counter
{
    pub fn new(ctx: &mut Context) -> GameResult<Counter>
    {
        use ggez::graphics::{Rect, DrawMode, MeshBuilder};

        let eighth = 1.0 / 8.0;

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

        let c = MeshBuilder::new()
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
                    eighth * 2.0,
                    eighth * 6.0,
                    eighth * 5.0,
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

        let d = MeshBuilder::new()
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
                    eighth * 4.0),
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

        let zero = MeshBuilder::new()
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
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 3.0,
                    eighth * 3.0,
                    eighth,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 4.0,
                    eighth * 4.0,
                    eighth,
                    eighth),
                (255, 255, 255).into())
            .build(ctx)?; // }

        let one = MeshBuilder::new()
            // {
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 2.0,
                    eighth,
                    eighth * 3.0,
                    eighth * 2.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth * 2.0,
                    eighth,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 3.0,
                    eighth * 3.0,
                    eighth * 2.0,
                    eighth * 3.0),
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

        let two = MeshBuilder::new()
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
                    eighth * 5.0,
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth * 2.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 4.0,
                    eighth * 4.0,
                    eighth * 2.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 2.0,
                    eighth * 5.0,
                    eighth * 3.0,
                    eighth),
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

        let three = MeshBuilder::new()
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
                    eighth * 5.0,
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

        let four = MeshBuilder::new()
            // {
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
                    eighth * 5.0,
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth * 5.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth * 3.0,
                    eighth * 2.0,
                    eighth * 2.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 3.0,
                    eighth * 4.0,
                    eighth * 2.0,
                    eighth),
                (255, 255, 255).into())
            .build(ctx)?; // }

        let five = MeshBuilder::new()
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
                    eighth,
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth * 2.0),
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
                    eighth,
                    eighth * 6.0,
                    eighth * 5.0,
                    eighth),
                (255, 255, 255).into())
            .build(ctx)?; // }

        let six = MeshBuilder::new()
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
                    eighth * 2.0,
                    eighth * 6.0,
                    eighth * 4.0,
                    eighth),
                (255, 255, 255).into())
            .build(ctx)?; // }

        let seven = MeshBuilder::new()
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
                    eighth * 5.0,
                    eighth * 2.0,
                    eighth * 2.0,
                    eighth),
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
                    eighth * 4.0,
                    eighth * 4.0,
                    eighth,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 3.0,
                    eighth * 5.0,
                    eighth * 2.0,
                    eighth * 2.0),
                (255, 255, 255).into())
            .build(ctx)?; // }

        let eight = MeshBuilder::new()
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
                    eighth * 2.0,
                    eighth * 3.0,
                    eighth * 4.0,
                    eighth),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth,
                    eighth * 4.0,
                    eighth * 2.0,
                    eighth * 2.0),
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

        let nine = MeshBuilder::new()
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
                    eighth * 2.0),
                (255, 255, 255).into())
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    eighth * 2.0,
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
                    eighth * 5.0),
                (255, 255, 255).into())
            .build(ctx)?; // }

        Ok(Counter
        {
            s: s,
            c: c,
            o: o,
            r: r,
            e: e,

            p: p,
            d: d,

            score: 0,
            score_digits: [0; 3],

            speed: 1,
            speed_digits: [0, 0, 1],

            zero: zero,
            one: one,
            two: two,
            three: three,
            four: four,
            five: five,
            six: six,
            seven: seven,
            eight: eight,
            nine: nine,
        })
    }

    pub fn update_score(&mut self, score: u32)
    {
        if score <= 999
        {
            self.score = score as u16;
        }
        else
        {
            self.score = 999;
        }

        self.score_digits = Self::to_digits(self.score);
    }

    pub fn update_speed(&mut self, speed: u64)
    {
        if speed <= 999
        {
            self.speed = speed as u16;
        }
        else
        {
            self.speed = 999;
        }

        self.speed_digits = Self::to_digits(self.speed);
    }

    fn to_digits(n: u16) -> [u8; 3]
    {
        let s = format!("{:03}", n);

        assert_eq!(s.len(), 3);

        let mut chars = s.chars();

        let from_char = |c: char|
        {
            match c
            {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => panic!("Not a digit"),
            }
        };

        [
            from_char(chars.next().unwrap()),
            from_char(chars.next().unwrap()),
            from_char(chars.next().unwrap()),
        ]
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
            (1.0 + super::BOARD_DIMENSIONS[0] as f32 + 1.0) * scale,
            (1.0 + 3.0 + 1.0 + 6.0 + 1.0) * scale,
        ];

        let text_scale = [
            scale * (3.0 / 5.0),
            scale * (3.0 / 5.0),
        ];

        draw(ctx, &self.s, DrawParam::default()
            .dest([
                offset[0] + start[0],
                offset[1] + start[1],
            ])
            .scale(text_scale))?;

        draw(ctx, &self.c, DrawParam::default()
            .dest([
                offset[0] + start[0] + 1.0 * text_scale[0],
                offset[1] + start[1],
            ])
            .scale(text_scale))?;

        draw(ctx, &self.o, DrawParam::default()
            .dest([
                offset[0] + start[0] + 2.0 * text_scale[0],
                offset[1] + start[1],
            ])
            .scale(text_scale))?;

        draw(ctx, &self.r, DrawParam::default()
            .dest([
                offset[0] + start[0] + 3.0 * text_scale[0],
                offset[1] + start[1]
            ])
            .scale(text_scale))?;

        draw(ctx, &self.e, DrawParam::default()
            .dest([
                offset[0] + start[0] + 4.0 * text_scale[0],
                offset[1] + start[1]
            ])
            .scale(text_scale))?;

        let get_digit = |n: u8|
        {
            match n
            {
                0 => &self.zero,
                1 => &self.one,
                2 => &self.two,
                3 => &self.three,
                4 => &self.four,
                5 => &self.five,
                6 => &self.six,
                7 => &self.seven,
                8 => &self.eight,
                9 => &self.nine,
                _ => panic!("digits cannot be greater than 9"),
            }
        };

        draw(ctx, get_digit(self.score_digits[0]), DrawParam::default()
            .dest([
                offset[0] + start[0],
                offset[1] + start[1] + 1.0 * text_scale[1],
            ])
            .scale([scale, scale]))?;

        draw(ctx, get_digit(self.score_digits[1]), DrawParam::default()
            .dest([
                offset[0] + start[0] + 1.0 * scale,
                offset[1] + start[1] + 1.0 * text_scale[1],
            ])
            .scale([scale, scale]))?;

        draw(ctx, get_digit(self.score_digits[2]), DrawParam::default()
            .dest([
                offset[0] + start[0] + 2.0 * scale,
                offset[1] + start[1] + 1.0 * text_scale[1],
            ])
            .scale([scale, scale]))?;

        let start = [
            start[0],
            start[1] + 1.0 * text_scale[0] + 1.0 * scale
        ];

        draw(ctx, &self.s, DrawParam::default()
            .dest([
                offset[0] + start[0],
                offset[1] + start[1],
            ])
            .scale(text_scale))?;

        draw(ctx, &self.p, DrawParam::default()
            .dest([
                offset[0] + start[0] + 1.0 * text_scale[0],
                offset[1] + start[1],
            ])
            .scale(text_scale))?;

        draw(ctx, &self.e, DrawParam::default()
            .dest([
                offset[0] + start[0] + 2.0 * text_scale[0],
                offset[1] + start[1],
            ])
            .scale(text_scale))?;

        draw(ctx, &self.e, DrawParam::default()
            .dest([
                offset[0] + start[0] + 3.0 * text_scale[0],
                offset[1] + start[1],
            ])
            .scale(text_scale))?;

        draw(ctx, &self.d, DrawParam::default()
            .dest([
                offset[0] + start[0] + 4.0 * text_scale[0],
                offset[1] + start[1],
            ])
            .scale(text_scale))?;

        draw(ctx, get_digit(self.speed_digits[0]), DrawParam::default()
            .dest([
                offset[0] + start[0],
                offset[1] + start[1] + 1.0 * text_scale[1],
            ])
            .scale([scale, scale]))?;

        draw(ctx, get_digit(self.speed_digits[1]), DrawParam::default()
            .dest([
                offset[0] + start[0] + 1.0 * scale,
                offset[1] + start[1] + 1.0 * text_scale[1],
            ])
            .scale([scale, scale]))?;

        draw(ctx, get_digit(self.speed_digits[2]), DrawParam::default()
            .dest([
                offset[0] + start[0] + 2.0 * scale,
                offset[1] + start[1] + 1.0 * text_scale[1],
            ])
            .scale([scale, scale]))?;

        Ok(())
    }
}
