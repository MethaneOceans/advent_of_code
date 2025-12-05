use ndarray::{Array2, ShapeBuilder, s};

use crate::SolverError;

pub fn solve(input: &str) -> Result<(String, String), SolverError> {
    let grid = parse_input(input)?;

    let part_1 = solve_part_1(&grid)?.to_string();
    let part_2 = solve_part_2(&grid)?.to_string();

    Ok((part_1, part_2))
}

fn parse_input(input: &str) -> Result<Array2<u8>, SolverError> {
    let lines = input.lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>();
    
    let width = lines[0].len();
    let height = lines.len();

    let values = input.lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(move |c| match c {
                    '@' => Ok(1),
                    '.' => Ok(0),
                    // _ => Err(SolverError::BadInput),
                    _ => panic!("Failed while parsing \"{}\"", &line),
                })
        })
        .flatten()
        .collect::<Result<Vec<_>, _>>()?;

    let grid = Array2::from_shape_vec(
        (width, height).strides((1,height)), 
        values
    ).unwrap();

    Ok(grid)
}

fn solve_part_1(grid: &Array2<u8>) -> Result<usize, SolverError> {
    let width = grid.dim().0;
    let height = grid.dim().1;
    let mut padded = Array2::from_elem((width + 2, height + 2), 0u8);
    let pad_slice = padded.slice_mut(s![1..=width, 1..=height]);
    
    grid.clone()
        .move_into(pad_slice);
    
    let mut count = 0;
    for window in padded.windows((3, 3)) {
        if *window.get((1, 1)).unwrap() as usize == 0 {
            continue;
        }

        let x: usize = window.flatten().iter().map(|i| *i as usize).sum::<usize>() - 1;

        if x < 4 {
            count += 1;
        }
    }

    Ok(count)
}

fn solve_part_2(grid: &Array2<u8>) -> Result<usize, SolverError> {
    let width = grid.dim().0;
    let height = grid.dim().1;
    let mut padded = Array2::from_elem((width + 2, height + 2), 0u8);
    let pad_slice = padded.slice_mut(s![1..=width, 1..=height]);
    
    grid.clone()
        .move_into(pad_slice);
    
    let mut count = 0;
    let mut removed_cells = true;

    while removed_cells {
        removed_cells = false;

        for y in 1..=height {
            for x in 1..=width {
                if *padded.get((x, y)).unwrap() as usize == 0 {
                    continue;
                }
    
                let adjacent: usize = [
                    *padded.get((x - 1, y - 1)).unwrap() as usize,
                    *padded.get((x    , y - 1)).unwrap() as usize,
                    *padded.get((x + 1, y - 1)).unwrap() as usize,
                    *padded.get((x - 1, y    )).unwrap() as usize,
                    *padded.get((x + 1, y    )).unwrap() as usize,
                    *padded.get((x - 1, y + 1)).unwrap() as usize,
                    *padded.get((x    , y + 1)).unwrap() as usize,
                    *padded.get((x + 1, y + 1)).unwrap() as usize,
                ].iter().sum();
    
                if adjacent < 4 {
                    count += 1;
                    *padded.get_mut((x, y)).unwrap() = 0;
                    removed_cells = true;
                }
            }
        }
    }


    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &'static str = "\
        ..@@.@@@@.\n\
        @@@.@.@.@@\n\
        @@@@@.@.@@\n\
        @.@@@@..@.\n\
        @@.@@@@.@@\n\
        .@@@@@@@.@\n\
        .@.@.@.@@@\n\
        @.@@@.@@@@\n\
        .@@@@@@@@.\n\
        @.@.@@@.@.";

    #[test]
    fn example_part_1() {
        let grid = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(grid.ncols(), 10);
        assert_eq!(grid.nrows(), 10);

        // for y in 0..grid.dim().1 {
        //     for x in 0..grid.dim().0 {
        //         print!("{}", grid.get((x, y)).unwrap());
        //     }
        //     println!();
        // }

        let result = solve_part_1(&grid).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn example_part_2() {
        let grid = parse_input(EXAMPLE_INPUT).unwrap();

        let result = solve_part_2(&grid).unwrap();
        assert_eq!(result, 43);
    }
}