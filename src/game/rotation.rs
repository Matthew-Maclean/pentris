#[derive(Copy, Clone)]
pub enum Rotation
{
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

impl Rotation
{
    pub fn clockwise(self) -> Rotation
    {
        match self
        {
            Rotation::Zero => Rotation::Ninety,
            Rotation::Ninety => Rotation::OneEighty,
            Rotation::OneEighty => Rotation::TwoSeventy,
            Rotation::TwoSeventy => Rotation::Zero,
        }
    }

    pub fn counterclockwise(self) -> Rotation
    {
        match self
        {
            Rotation::TwoSeventy => Rotation::OneEighty,
            Rotation::OneEighty => Rotation::Ninety,
            Rotation::Ninety => Rotation::Zero,
            Rotation::Zero => Rotation::TwoSeventy,
        }
    }
}
