use crate::{utils};

struct cell{
    value: u32,
    marked: bool,
}

struct board{
    rows: [[cell]],
}

#[cfg(test)]
mod tests {
    use crate::{day4, utils};

    #[test]
    fn part1() {
        println!("--- DAY 4-1 ----");
        let input = utils::read_file("inputs/4.txt");
        let board = board{
            rows: [
                [5,]
            ]
        }
    }

    #[test]
    fn part2() {
        println!("--- DAY 4-2 ----");
        let input = utils::read_file("inputs/4.txt");
    }
}