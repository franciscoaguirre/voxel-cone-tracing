pub const CHILDREN_PER_NODE: u32 = 8;

#[derive(Clone, Copy)]
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
            _ => panic!("Wrong Axis value"),
        }
    }
}
