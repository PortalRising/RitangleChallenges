use crate::grid::Grid;

pub fn clue_one(grid: &Grid) -> bool {
    grid.row(0).iter().all(|i| *i == 0)
}