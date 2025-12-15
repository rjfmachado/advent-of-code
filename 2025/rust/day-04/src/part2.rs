const ADJACENT: usize = 1;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut removed: usize = 0;

    loop {
        let mut accessible: usize = 0;
        //mark for removal
        for row in 0..grid.len() {
            for column in 0..grid[row].len() {
                if grid[row][column] == '@' {
                    if get_surrounding(
                        &grid, row, column, ADJACENT,
                    ) < 4
                    {
                        accessible += 1;
                        grid[row][column] = 'x'
                    }
                }
            }
        }

        //mark removed
        for row in 0..grid.len() {
            for column in 0..grid[row].len() {
                if grid[row][column] == 'x' {
                    grid[row][column] = '.'
                }
            }
        }

        if accessible == 0 {
            break;
        } else {
            removed += accessible;
        }
    }

    Ok(removed.to_string())
}

fn get_surrounding(
    source: &Vec<Vec<char>>,
    row: usize,
    column: usize,
    size: usize,
) -> usize {
    let mut surrounding: usize = 0;
    let row_count: usize = source.len();
    let column_count: usize = source[0].len();

    for x in -(size as isize)..=(size as isize) {
        let target_row: Option<usize> =
            row.checked_add_signed(x);

        match target_row {
            Some(row) => {
                for y in -(size as isize)..=(size as isize)
                {
                    let target_column: Option<usize> =
                        column.checked_add_signed(y);

                    match target_column {
                        Some(column) => {
                            if row < row_count
                                && column < column_count
                            {
                                if source[row][column]
                                    == '@'
                                    || source[row][column]
                                        == 'x'
                                {
                                    surrounding += 1;
                                }
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }
    }
    surrounding - 1 // do not count the center (itself)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("43", process(input)?);
        Ok(())
    }
}
