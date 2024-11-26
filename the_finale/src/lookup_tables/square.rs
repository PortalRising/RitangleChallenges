/// A table of all squares
pub struct SquareTable;

impl SquareTable {
    /// All square numbers between 1 and 1000
    const SQUARES: [usize; 31] = [
        1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361, 400,
        441, 484, 529, 576, 625, 676, 729, 784, 841, 900, 961,
    ];

    pub fn squares() -> &'static [usize] {
        &Self::SQUARES
    }

    pub fn is_square(number: usize) -> bool {
        Self::SQUARES.binary_search(&number).is_ok()
    }
}
