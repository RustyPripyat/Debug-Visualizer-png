/// Coordinate struct
/// The `Coordinate` struct is used to define the coordinates of a tile.
///
/// # Usage
/// ```
/// use robotics_lib::world::coordinates::Coordinate;
/// ```
///
/// # Example
/// ```
/// use robotics_lib::world::coordinates::Coordinate;
/// // let coordinate = Coordinate::new(0, 0);
/// ```
#[derive(Clone, Debug)]
pub struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    pub(crate) fn new(row: usize, col: usize) -> Self {
        Coordinate { row, col }
    }
    pub fn get_row(&self) -> usize {
        self.row
    }
    pub fn get_col(&self) -> usize {
        self.col
    }
}
