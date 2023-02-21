#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl From<Direction> for Axis {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Top | Direction::Bottom => Axis::Y,
            Direction::West | Direction::East => Axis::X,
            Direction::North | Direction::South => Axis::Z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Top,
    Bottom,
    West,
    East,
    North,
    South,
}

impl IntoIterator for Direction {
    type Item = Direction;

    type IntoIter = DirectionIter;

    fn into_iter(self) -> Self::IntoIter {
        DirectionIter {
            count: 0,
            last: self,
        }
    }
}

impl Direction {
    pub const TOP_NORMAL: [f32; 3] = [0.0, 1.0, 0.0];
    pub const BOTTOM_NORMAL: [f32; 3] = [0.0, -1.0, 0.0];
    pub const WEST_NORMAL: [f32; 3] = [-1.0, 0.0, 0.0];
    pub const EAST_NORMAL: [f32; 3] = [1.0, 0.0, 0.0];
    pub const NORTH_NORMAL: [f32; 3] = [0.0, 0.0, 1.0];
    pub const SOUTH_NORMAL: [f32; 3] = [0.0, 0.0, -1.0];

    /// This probably should be renamed as this just returns
    /// a vector in given direction.
    pub fn normal(self) -> [f32; 3] {
        match self {
            Direction::Top => Self::TOP_NORMAL,
            Direction::Bottom => Self::BOTTOM_NORMAL,
            Direction::West => Self::WEST_NORMAL,
            Direction::East => Self::EAST_NORMAL,
            Direction::North => Self::NORTH_NORMAL,
            Direction::South => Self::SOUTH_NORMAL,
        }
    }
}

pub struct DirectionIter {
    count: usize,
    last: Direction,
}

impl Default for DirectionIter {
    fn default() -> Self {
        Self {
            count: 0,
            last: Direction::South,
        }
    }
}

impl Iterator for DirectionIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 6 {
            self.count += 1;
            let next = match self.last {
                Direction::Top => Direction::Bottom,
                Direction::Bottom => Direction::West,
                Direction::West => Direction::East,
                Direction::East => Direction::North,
                Direction::North => Direction::South,
                Direction::South => Direction::Top,
            };
            self.last = next;
            return Some(next);
        }
        None
    }
}

impl DirectionIter {
    pub fn new(start: Direction) -> Self {
        start.into_iter()
    }
}
