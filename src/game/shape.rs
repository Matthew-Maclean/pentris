use ggez::
{
    Context,
    GameResult,
    graphics::
    {
        Mesh,
        Color as Colour,
    },
};

use super::Rotation;

pub const NUM_SHAPES: u8 = 12;

#[derive(Copy, Clone)]
pub enum Shape
{
    F,
    I,
    L,
    N,
    P,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Shape
{
    pub fn numbered(n: u8) -> Shape
    {
        match n % NUM_SHAPES
        {
            0 => Shape::F,
            1 => Shape::I,
            2 => Shape::L,
            3 => Shape::N,
            4 => Shape::P,
            5 => Shape::T,
            6 => Shape::U,
            7 => Shape::V,
            8 => Shape::W,
            9 => Shape::X,
            10 => Shape::Y,
            11 => Shape::Z,
            _ => unreachable!(),
        }
    }

    pub fn colour(self) -> Colour
    {
        match self
        {
            Shape::F => (255, 64 , 64 ),
            Shape::I => (255, 159, 64 ),
            Shape::L => (255, 255, 64 ),
            Shape::N => (159, 255, 64 ),
            Shape::P => (64 , 255, 64 ),
            Shape::T => (64 , 255, 159),
            Shape::U => (64 , 255, 255),
            Shape::V => (64 , 160, 255),
            Shape::W => (64 , 64 , 255),
            Shape::X => (159, 64 , 255),
            Shape::Y => (255, 64 , 255),
            Shape::Z => (255, 64 , 159),
        }.into()
    }
}

pub struct ShapeData
{
    tiles: ShapeTiles,
    meshes: ShapeMeshes,
}

impl ShapeData
{
    pub fn init(ctx: &mut Context) -> GameResult<ShapeData>
    {
        let tiles = ShapeTiles
        {
            f: TileGroup
            {
                zero: ShapeData::parse([
                    "     ",
                    "  +  ",
                    " ++  ",
                    "  ++ ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    "  +  ",
                    " +++ ",
                    " +   ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    "     ",
                    " ++  ",
                    "  ++ ",
                    "  +  ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    "   + ",
                    " +++ ",
                    "  +  ",
                    "     ",
                ]),
            },
            f_flip: TileGroup
            {
                zero: ShapeData::parse([
                    "     ",
                    "  +  ",
                    "  ++ ",
                    " ++  ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    " +   ",
                    " +++ ",
                    "  +  ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    "     ",
                    "  ++ ",
                    " ++  ",
                    "  +  ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    "  +  ",
                    " +++ ",
                    "   + ",
                    "     ",
                ]),
            },

            i: TileGroup
            {
                zero: ShapeData::parse([
                    "  +  ",
                    "  +  ",
                    "  +  ",
                    "  +  ",
                    "  +  ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    "     ",
                    "+++++",
                    "     ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    "  +  ",
                    "  +  ",
                    "  +  ",
                    "  +  ",
                    "  +  ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    "     ",
                    "+++++",
                    "     ",
                    "     ",
                ]),
            },

            l: TileGroup
            {
                zero: ShapeData::parse([
                    "  +  ",
                    "  +  ",
                    "  +  ",
                    "  ++ ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    "++++ ",
                    "+    ",
                    "     ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    " ++  ",
                    "  +  ",
                    "  +  ",
                    "  +  ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    "   + ",
                    "++++ ",
                    "     ",
                    "     ",
                ]),
            },
            l_flip: TileGroup
            {
                zero: ShapeData::parse([
                    "  +  ",
                    "  +  ",
                    "  +  ",
                    " ++  ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    "+    ",
                    "++++ ",
                    "     ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    "  ++ ",
                    "  +  ",
                    "  +  ",
                    "  +  ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    "++++ ",
                    "   + ",
                    "     ",
                    "     ",
                ]),
            },

            n: TileGroup
            {
                zero: ShapeData::parse([
                    "  +  ",
                    "  +  ",
                    " ++  ",
                    " +   ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    "++   ",
                    " +++ ",
                    "     ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    "  +  ",
                    " ++  ",
                    " +   ",
                    " +   ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    "+++  ",
                    "  ++ ",
                    "     ",
                    "     ",
                ]),
            },
            n_flip: TileGroup
            {
                zero: ShapeData::parse([
                    "  +  ",
                    "  +  ",
                    "  ++ ",
                    "   + ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    " +++ ",
                    "++   ",
                    "     ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    " +   ",
                    " ++  ",
                    "  +  ",
                    "  +  ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    "  ++ ",
                    "+++  ",
                    "     ",
                    "     ",
                ]),
            },

            p: TileGroup
            {
                zero: ShapeData::parse([
                    "     ",
                    "  ++ ",
                    "  ++ ",
                    "  +  ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    " +++ ",
                    "  ++ ",
                    "     ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    "     ",
                    "  +  ",
                    " ++  ",
                    " ++  ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    "     ",
                    " ++  ",
                    " +++ ",
                    "     ",
                ]),
            },
            p_flip: TileGroup
            {
                zero: ShapeData::parse([
                    "     ",
                    " ++  ",
                    " ++  ",
                    "  +  ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    "     ",
                    "  ++ ",
                    " +++ ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    "     ",
                    "  +  ",
                    "  ++ ",
                    "  ++ ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    " +++ ",
                    " ++  ",
                    "     ",
                    "     ",
                ]),
            },

            t: TileGroup
            {
                zero: ShapeData::parse([
                    "     ",
                    " +++ ",
                    "  +  ",
                    "  +  ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    "   + ",
                    " +++ ",
                    "   + ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    "     ",
                    "  +  ",
                    "  +  ",
                    " +++ ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    " +   ",
                    " +++ ",
                    " +   ",
                    "     ",
                ]),
            },

            u: TileGroup
            {
                zero: ShapeData::parse([
                    "     ",
                    " + + ",
                    " +++ ",
                    "     ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    "  ++ ",
                    "  +  ",
                    "  ++ ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    "     ",
                    "     ",
                    " +++ ",
                    " + + ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    " ++  ",
                    "  +  ",
                    " ++  ",
                    "     ",
                ]),
            },

            v: TileGroup
            {
                zero: ShapeData::parse([
                    "     ",
                    " +   ",
                    " +   ",
                    " +++ ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    " +++ ",
                    " +   ",
                    " +   ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    "     ",
                    " +++ ",
                    "   + ",
                    "   + ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    "   + ",
                    "   + ",
                    " +++ ",
                    "     ",
                ]),
            },

            w: TileGroup
            {
                zero: ShapeData::parse([
                    "     ",
                    " +   ",
                    " ++  ",
                    "  ++ ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    "  ++ ",
                    " ++  ",
                    " +   ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    "     ",
                    " ++  ",
                    "  ++ ",
                    "   + ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    "   + ",
                    "  ++ ",
                    " ++  ",
                    "     ",
                ]),
            },

            x: TileGroup
            {
                zero: ShapeData::parse([
                    "     ",
                    "  +  ",
                    " +++ ",
                    "  +  ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    "  +  ",
                    " +++ ",
                    "  +  ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    "     ",
                    "  +  ",
                    " +++ ",
                    "  +  ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    "  +  ",
                    " +++ ",
                    "  +  ",
                    "     ",
                ]),
            },

            y: TileGroup
            {
                zero: ShapeData::parse([
                    "  +  ",
                    " ++  ",
                    "  +  ",
                    "  +  ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    "  +  ",
                    "++++ ",
                    "     ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    " +   ",
                    " +   ",
                    " ++  ",
                    " +   ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    "++++ ",
                    " +   ",
                    "     ",
                    "     ",
                ]),
            },
            y_flip: TileGroup
            {
                zero: ShapeData::parse([
                    "  +  ",
                    "  ++ ",
                    "  +  ",
                    "  +  ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    "++++ ",
                    "  +  ",
                    "     ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    "  +  ",
                    "  +  ",
                    " ++  ",
                    "  +  ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    " +   ",
                    "++++ ",
                    "     ",
                    "     ",
                ]),
            },

            z: TileGroup
            {
                zero: ShapeData::parse([
                    "     ",
                    " ++  ",
                    "  +  ",
                    "  ++ ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    "   + ",
                    " +++ ",
                    " +   ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    "     ",
                    " ++  ",
                    "  +  ",
                    "  ++ ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    "   + ",
                    " +++ ",
                    " +   ",
                    "     ",
                ]),
            },
            z_flip: TileGroup
            {
                zero: ShapeData::parse([
                    "     ",
                    "  ++ ",
                    "  +  ",
                    " ++  ",
                    "     ",
                ]),
                ninety: ShapeData::parse([
                    "     ",
                    " +   ",
                    " +++ ",
                    "   + ",
                    "     ",
                ]),
                one_eighty: ShapeData::parse([
                    "     ",
                    "  ++ ",
                    "  +  ",
                    " ++  ",
                    "     ",
                ]),
                two_seventy: ShapeData::parse([
                    "     ",
                    " +   ",
                    " +++ ",
                    "   + ",
                    "     ",
                ]),
            },
        };

        Ok(ShapeData
        {
            meshes: ShapeMeshes
            {
                f: ShapeData::make_mesh_group(ctx, &tiles.f)?,
                f_flip: ShapeData::make_mesh_group(ctx, &tiles.f_flip)?,

                i: ShapeData::make_mesh_group(ctx, &tiles.i)?,

                l: ShapeData::make_mesh_group(ctx, &tiles.l)?,
                l_flip: ShapeData::make_mesh_group(ctx, &tiles.l_flip)?,

                n: ShapeData::make_mesh_group(ctx, &tiles.n)?,
                n_flip: ShapeData::make_mesh_group(ctx, &tiles.n_flip)?,

                p: ShapeData::make_mesh_group(ctx, &tiles.p)?,
                p_flip: ShapeData::make_mesh_group(ctx, &tiles.p_flip)?,

                t: ShapeData::make_mesh_group(ctx, &tiles.t)?,

                u: ShapeData::make_mesh_group(ctx, &tiles.u)?,

                v: ShapeData::make_mesh_group(ctx, &tiles.v)?,

                w: ShapeData::make_mesh_group(ctx, &tiles.w)?,

                x: ShapeData::make_mesh_group(ctx, &tiles.x)?,

                y: ShapeData::make_mesh_group(ctx, &tiles.y)?,
                y_flip: ShapeData::make_mesh_group(ctx, &tiles.y_flip)?,

                z: ShapeData::make_mesh_group(ctx, &tiles.z)?,
                z_flip: ShapeData::make_mesh_group(ctx, &tiles.z_flip)?,
            },
            tiles: tiles,
        })
    }

    pub fn get_tiles(&self, shape: Shape, rot: Rotation, flip: bool) -> &Tiles
    {
        match shape
        {
            Shape::F =>
                if flip { self.tiles.f_flip.get(rot) }
                else { self.tiles.f.get(rot) },
            Shape::I => self.tiles.i.get(rot),
            Shape::L =>
                if flip { self.tiles.l_flip.get(rot) }
                else { self.tiles.l.get(rot) },
            Shape::N =>
                if flip { self.tiles.n_flip.get(rot) }
                else { self.tiles.n.get(rot) },
            Shape::P =>
                if flip { self.tiles.p_flip.get(rot) }
                else { self.tiles.p.get(rot) },
            Shape::T => self.tiles.t.get(rot),
            Shape::U => self.tiles.u.get(rot),
            Shape::V => self.tiles.v.get(rot),
            Shape::W => self.tiles.w.get(rot),
            Shape::X => self.tiles.x.get(rot),
            Shape::Y =>
                if flip { self.tiles.y_flip.get(rot) }
                else { self.tiles.y.get(rot) },
            Shape::Z =>
                if flip { self.tiles.z_flip.get(rot) }
                else { self.tiles.z.get(rot) },
        }
    }

    pub fn get_mesh(&self, shape: Shape, rot: Rotation, flip: bool) -> &Mesh
    {
        match shape
        {
            Shape::F =>
                if flip { self.meshes.f_flip.get(rot) }
                else { self.meshes.f.get(rot) },
            Shape::I => self.meshes.i.get(rot),
            Shape::L =>
                if flip { self.meshes.l_flip.get(rot) }
                else { self.meshes.l.get(rot) },
            Shape::N =>
                if flip { self.meshes.n_flip.get(rot) }
                else { self.meshes.n.get(rot) },
            Shape::P =>
                if flip { self.meshes.p_flip.get(rot) }
                else { self.meshes.p.get(rot) },
            Shape::T => self.meshes.t.get(rot),
            Shape::U => self.meshes.u.get(rot),
            Shape::V => self.meshes.v.get(rot),
            Shape::W => self.meshes.w.get(rot),
            Shape::X => self.meshes.x.get(rot),
            Shape::Y =>
                if flip { self.meshes.y_flip.get(rot) }
                else { self.meshes.y.get(rot) },
            Shape::Z =>
                if flip { self.meshes.z_flip.get(rot) }
                else { self.meshes.z.get(rot) },
        }
    }

    fn parse(lines: [&str; 5]) -> Tiles
    {
        let mut vec = Vec::new();

        parse_line(lines[0],  2, &mut vec);
        parse_line(lines[1],  1, &mut vec);
        parse_line(lines[2],  0, &mut vec);
        parse_line(lines[3], -1, &mut vec);
        parse_line(lines[4], -2, &mut vec);

        let mut tiles = [[0; 2]; 5];

        let mut i = 0;

        for t in vec
        {
            if let Some(tile) = t
            {
                tiles[i] = tile;
                i += 1;
                if i >= 5
                {
                    break;
                }
            }
        }

        return tiles;

        fn parse_line(line: &str, y: i32, tiles: &mut Vec<Option<[i32; 2]>>)
        {
            assert!(line.len() == 5);

            let chars = line.chars().collect::<Vec<_>>();

            tiles.push(if chars[0] != ' ' { Some([-2, y]) } else { None });
            tiles.push(if chars[1] != ' ' { Some([-1, y]) } else { None });
            tiles.push(if chars[2] != ' ' { Some([ 0, y]) } else { None });
            tiles.push(if chars[3] != ' ' { Some([ 1, y]) } else { None });
            tiles.push(if chars[4] != ' ' { Some([ 2, y]) } else { None });
        }
    }

    fn make_mesh(ctx: &mut Context, tiles: &Tiles)
        -> GameResult<Mesh>
    {
        use ggez::graphics::
        {
            Rect,
            DrawMode,
            MeshBuilder
        };

        let mut mesh = MeshBuilder::new();

        for tile in tiles.iter()
        {
                let x = tile[0] as f32 + 2.0;
                let y = -(tile[1] as f32) + 2.0;

                mesh.rectangle(
                    DrawMode::fill(),
                    Rect::new(x, y, 1.0, 1.0),
                    (255, 255, 255).into());
        }

        mesh.build(ctx)
    }

    fn make_mesh_group(ctx: &mut Context, tiles: &TileGroup)
        -> GameResult<MeshGroup>
    {
        Ok(MeshGroup
        {
            zero: ShapeData::make_mesh(ctx, &tiles.zero)?,
            ninety: ShapeData::make_mesh(ctx, &tiles.ninety)?,
            one_eighty: ShapeData::make_mesh(ctx, &tiles.one_eighty)?,
            two_seventy: ShapeData::make_mesh(ctx, &tiles.two_seventy)?,
        })
    }
}

pub type Tiles = [[i32; 2]; 5];

pub fn stamp_tiles(tiles: &Tiles, pos: [i32; 2]) -> [[i32; 2]; 5]
{
    let mut i = 0;

    let mut stamp = [[0; 2]; 5];

    for diff in tiles.iter()
    {
        stamp[i] = [pos[0] + diff[0], pos[1] + diff[1]];
        i += 1;
    }

    stamp
}

pub struct TileGroup
{
    zero: Tiles,
    ninety: Tiles,
    one_eighty: Tiles,
    two_seventy: Tiles,
}

impl TileGroup
{
    fn get(&self, rot: Rotation) -> &Tiles
    {
        match rot
        {
            Rotation::Zero => &self.zero,
            Rotation::Ninety => &self.ninety,
            Rotation::OneEighty => &self.one_eighty,
            Rotation::TwoSeventy => &self.two_seventy,
        }
    }
}

pub struct ShapeTiles
{
    f: TileGroup,
    f_flip: TileGroup,

    i: TileGroup,

    l: TileGroup,
    l_flip: TileGroup,

    n: TileGroup,
    n_flip: TileGroup,

    p: TileGroup,
    p_flip: TileGroup,

    t: TileGroup,

    u: TileGroup,

    v: TileGroup,

    w: TileGroup,

    x: TileGroup,

    y: TileGroup,
    y_flip: TileGroup,

    z: TileGroup,
    z_flip: TileGroup,
}

pub struct MeshGroup
{
    zero: Mesh,
    ninety: Mesh,
    one_eighty: Mesh,
    two_seventy: Mesh,
}

impl MeshGroup
{
    fn get(&self, rot: Rotation) -> &Mesh
    {
        match rot
        {
            Rotation::Zero => &self.zero,
            Rotation::Ninety => &self.ninety,
            Rotation::OneEighty => &self.one_eighty,
            Rotation::TwoSeventy => &self.two_seventy,
        }
    }
}

pub struct ShapeMeshes
{
    f: MeshGroup,
    f_flip: MeshGroup,

    i: MeshGroup,

    l: MeshGroup,
    l_flip: MeshGroup,

    n: MeshGroup,
    n_flip: MeshGroup,

    p: MeshGroup,
    p_flip: MeshGroup,

    t: MeshGroup,

    u: MeshGroup,

    v: MeshGroup,

    w: MeshGroup,

    x: MeshGroup,

    y: MeshGroup,
    y_flip: MeshGroup,

    z: MeshGroup,
    z_flip: MeshGroup,
}
