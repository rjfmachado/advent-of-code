use itertools::Itertools;
use nom::Input;

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
    None,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut data: Vec<_> = input.lines().collect();

    let operations: Vec<Operation> = data
        .last()
        .unwrap()
        .split(" ")
        .filter(|operation| !operation.is_empty())
        .map(|operation| match operation {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            _ => Operation::None,
        })
        .collect();

    data.truncate(&data.len() - 1);
    //dbg!(&data);

    let x: Vec<Vec<_>> = data
        .iter()
        .map(|lines| {
            lines
                .chars()
                .enumerate()
                .filter(|x| x.0 + 1 % 4 != 0)
                .map(|x| x.1)
                .collect()
        })
        .collect();

    dbg!(&x);

    // let numbers: Vec<Vec<u64>> = x
    //     .into_iter()
    //     .map(|numbers| {
    //         numbers
    //             .split(" ")
    //             //.filter(|number| !number.is_empty())
    //             .map(|n| {
    //                 n.parse::<u64>().unwrap_or_default()
    //             })
    //             .collect()
    //     })
    //     .collect();

    //dbg!(&operations);
    // dbg!(&numbers);

    // let row_size: usize = numbers[0].len();
    // let col_size: usize = numbers.len();

    let mut result: usize = 0;
    // for x in (0..row_size).rev() {
    //     let mut column: usize = 0;
    //     for y in (0..col_size).rev() {
    //         match &operations[x] {
    //             Operation::Add => column += &numbers[y][x],
    //             Operation::Multiply => {
    //                 if column == 0 {
    //                     column += &numbers[y][x]
    //                 } else {
    //                     column *= &numbers[y][x]
    //                 }
    //             }
    //             Operation::None => {}
    //         }
    //     }
    //     result += column;
    // }
    // for x in 0..row_size as usize {
    //     let mut column: u64 = 0;
    //     for y in 0..(col_size) as usize {
    //         //dbg!(&numbers[y][x]);
    //         match &operations[x] {
    //             Operation::Add => column += &numbers[y][x],
    //             Operation::Multiply => {
    //                 if column == 0 {
    //                     column += &numbers[y][x]
    //                 } else {
    //                     column *= &numbers[y][x]
    //                 }
    //             }
    //             Operation::None => {}
    //         }
    //     }
    //     result += column;
    // }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";
        assert_eq!("4277556", process(input)?);
        Ok(())
    }
}
