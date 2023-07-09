pub const CHILDREN_PER_NODE: u32 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Into<u32> for Axis {
    fn into(self) -> u32 {
        match self {
            Axis::X => 0,
            Axis::Y => 1,
            Axis::Z => 2,
        }
    }
}

impl Into<i32> for Axis {
    fn into(self) -> i32 {
        match self {
            Axis::X => 0,
            Axis::Y => 1,
            Axis::Z => 2,
        }
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
