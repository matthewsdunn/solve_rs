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

/// Returns a bool indicating whether or not the number passed in as
/// 'number' is found in the row passed in as 'row'
fn in_row(grid: &mut [[u8; 9]; 9], row: usize, number: u8) -> bool {
    return grid[row].iter().any(|v: &u8| *v == number);
}

/// Returns a bool indicating whether or not the number passed in as
/// 'number' is found in the column passed in as 'col'
fn in_col(grid: &mut [[u8; 9]; 9], col: usize, number: u8) -> bool {
    // TODO: Or faster to do a 'collect' into a new array and search that?
    for x in 0..9 {
        if grid[x][col] == number {
            return true;
        }
    }

    return false;
}

/// Returns a bool indicating whether or not the number passed in as
/// 'number' is found in the 3x3 section of the 9x9 grid in which
/// row/col are found
fn in_box(grid: &mut [[u8; 9]; 9], row: usize, col: usize, number: u8) -> bool {
    // because, given a 9x9 Sudoku grid,
    // each 'box' will be of size 3x3
    for x in 0..3 {
        for y in 0..3 {
            if grid[x + row][y + col] == number {
                return true;
            }
        }
    }

    return false;
}

/// Returns a bool indicating whether the proposed assignment of a particular
/// number to a particular cell would violate any of the constraints of the
/// puzzle, as expressed via the functions above
fn is_safe(grid: &mut [[u8; 9]; 9], row: usize, col: usize, number: u8) -> bool {
    !in_row(grid, row, number)
        && !in_col(grid, col, number)
        && !in_box(grid, row - row % 3, col - col % 3, number)
}

fn get_next_location(grid: &mut [[u8; 9]; 9]) -> (usize, usize) {
    for x in 0..9 {
        for y in 0..9 {
            if grid[x][y] == 0 {
                return (x, y)
            }
        }
    }

    // TODO: Make this a real enum (Option<int> ?)
    return (42, 42)
}

/// The core solving algorithm that brute force, recursively searches through
/// all options. Returns a bool indicating whether or not it was capable of
/// successfully solving the puzzle.
fn solve(grid: &mut [[u8; 9]; 9]) -> bool {
    let loc = get_next_location(grid);
    match loc {
        (42, 42) => return true,
        (x, y) => {
            for n in 1..=9 {
                if is_safe(grid, x, y, n) {
                    grid[x][y] = n;

                    if solve(grid) {
                        return true
                    }

                    // reset the value
                    grid[x][y] = 0;
                }
            }
        }
    }

    return false
}

/// Entry point
fn main() {
    let mut grid: [[u8; 9]; 9] = [
        [0, 0, 3, 0, 0, 5, 0, 0, 9],
        [0, 8, 0, 0, 7, 0, 0, 3, 0],
        [6, 0, 0, 3, 0, 0, 2, 0, 0],
        [5, 0, 0, 2, 0, 0, 4, 0, 0],
        [0, 9, 0, 0, 6, 0, 0, 7, 0],
        [0, 0, 7, 0, 0, 1, 0, 0, 2],
        [0, 0, 6, 0, 0, 4, 0, 0, 5],
        [0, 4, 0, 0, 1, 0, 0, 2, 0],
        [3, 0, 0, 9, 0, 0, 8, 0, 0],
    ];

    assert_eq!(solve(&mut grid), true);

    println!("Solved puzzle is:\n{:?}", grid);
}
