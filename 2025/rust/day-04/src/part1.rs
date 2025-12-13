use std::ops::Add;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut accessible: u64 = 0;
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    //dbg!(&grid);
    //dbg!(&grid[0][0]);

    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            if get_surrounding(&grid, row, column, 2) < 4 {
                accessible += 1
            }
        }
    }

    Ok(accessible.to_string())
}

fn get_surrounding(
    source: &Vec<Vec<char>>,
    row: usize,
    column: usize,
    size: isize,
) -> u64 {
    let mut surrounding: usize = 0;
    let row_count: usize = source.len();
    let column_count: usize = source[row].len();

    for x in -size..size {
        let target_row =
            (row as isize).checked_add(x).unwrap_or(0);

        if row as isize + x >= 0
            && row as isize + x <= row_count as isize
        {
            for y in -size..size {
                if column as isize + y >= 0
                    && column as isize + y
                        <= column_count as isize
                {
                    if source[row + x as usize]
                        [column + y as usize]
                        == '@'
                        && x != 0
                        && y != 0
                    {
                        surrounding += 1;
                    }
                }
            }
        }
    }

    surrounding as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
