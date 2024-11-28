use std::sync::LazyLock;

/// A table of all squares
pub struct SquareTable;

impl SquareTable {
    pub fn squares() -> &'static [usize] {
        static SQUARES: LazyLock<Vec<usize>> = LazyLock::new(|| {
            let mut squares = Vec::with_capacity(45);

            for i in 1..45usize {
                squares.push(i.pow(2));
            }

            squares.sort_unstable();

            println!("{:?}", squares);

            squares
        });

        &SQUARES
    }

    pub fn is_square(number: usize) -> bool {
        Self::squares().binary_search(&number).is_ok()
    }
}
