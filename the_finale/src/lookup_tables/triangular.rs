/// A table of all triangular numbers
pub struct TriangularTable;

impl TriangularTable {
    /// All triangular numbers between 1 and 1000
    const TRIANGULARS: [usize; 44] = [
        1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210, 231,
        253, 276, 300, 325, 351, 378, 406, 435, 465, 496, 528, 561, 595, 630, 666, 703, 741, 780,
        820, 861, 903, 946, 990,
    ];

    pub fn triangulars() -> &'static [usize] {
        &Self::TRIANGULARS
    }

    pub fn is_triangular(number: usize) -> bool {
        Self::TRIANGULARS.binary_search(&number).is_ok()
    }
}
