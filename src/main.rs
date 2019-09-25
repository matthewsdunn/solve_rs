/// solve_rs
///
/// solve_rs is an initial attempt at a brute-force implementation of a
/// Sudoku puzzle solver. The intent is to move eventually from a brute-force
/// approach to an ML-based approach that can use heuristics developed through
/// supervised learning, etc.
///
/// Copyright 2019 - Licensed under the MIT License
///
/// This program is being provided 'as is', without warranty of any kind,
/// expressed or implied, including, but not limited to, the implied warranties
/// of merchantability and fitness for a particular purpose.
use ndarray::prelude::*;

/// Returns a bool indicating whether or not the number passed in as
/// 'number' is found in the row passed in as 'row'
fn in_row(grid: &mut Array2<u8>, row: usize, number: u8) -> bool {
    let r = grid.slice(s![row, ..]);
    return r.iter().any(|v: &u8| *v == number);
}

/// Returns a bool indicating whether or not the number passed in as
/// 'number' is found in the column passed in as 'col'
fn in_col(grid: &mut Array2<u8>, col: usize, number: u8) -> bool {
    let c = grid.slice(s![.., col]);
    return c.iter().any(|v: &u8| *v == number);
}

/// Returns a bool indicating whether or not the number passed in as
/// 'number' is found in the 3x3 section of the grid in which row/col
/// are found
fn in_box(grid: &mut Array2<u8>, row: usize, col: usize, number: u8) -> bool {
    return true;
}

fn main() {
    let mut grid = array![
        [0, 1, 2, 3, 4, 5, 6, 7, 8],
        [0, 1, 2, 3, 4, 5, 6, 7, 8],
        [0, 1, 2, 3, 4, 5, 6, 7, 8],
        [0, 1, 2, 3, 4, 5, 6, 7, 8],
        [0, 1, 2, 3, 4, 5, 6, 7, 8],
        [0, 1, 2, 3, 4, 5, 6, 7, 8],
        [0, 1, 2, 3, 4, 5, 6, 7, 8],
        [0, 1, 2, 3, 4, 5, 6, 7, 8],
        [0, 1, 2, 3, 4, 5, 6, 7, 8],
    ];

    assert_eq!(grid.shape(), &[9, 9]);

    assert_eq!(in_row(&mut grid, 1, 4), true);
    assert_eq!(in_col(&mut grid, 1, 1), true);
    assert_eq!(in_box(&mut grid, 1, 1, 1), true);
}
