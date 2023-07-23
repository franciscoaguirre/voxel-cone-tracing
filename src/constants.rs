pub const CHILDREN_PER_NODE: u32 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

macro_rules! impl_into_number_axis {
    ($type:ty) => {
        impl Into<$type> for Axis {
            fn into(self) -> $type {
                match self {
                    Axis::X => 0,
                    Axis::Y => 1,
                    Axis::Z => 2,
                }
            }
        }
    };
}

impl_into_number_axis!(u32);
impl_into_number_axis!(i32);
impl_into_number_axis!(usize);

impl Axis {
    pub fn all_axis() -> [Self; 3] {
        [Self::X, Self::Y, Self::Z]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Sign {
    Pos,
    Neg,
}

impl Into<i32> for Sign {
    fn into(self) -> i32 {
        match self {
            Sign::Pos => 1,
            Sign::Neg => -1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Direction {
    pub axis: Axis,
    pub sign: Sign,
}

impl Direction {
    pub fn new(axis: Axis, sign: Sign) -> Self {
        Self { axis, sign }
    }
}
