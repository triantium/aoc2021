//use crate::{utils};

#[derive(Copy, Clone)]
struct Cell{
    value: u32,
    marked: bool,
}

struct BingoBoard{
    rows: [[Cell;5];5]
}

impl BingoBoard {
    fn new() -> BingoBoard{
        return BingoBoard{
            rows: [[Cell{value:0,marked:false};5];5]
        }
    }

    fn is_finished(&self) -> bool {
        //(Diagonals don't count.)
        return self.check_rows()
            || self.check_columns();
    }

    fn check_row(cells: &[Cell;5]) -> bool {
        for cell in cells.iter() {
            if ! cell.marked {
                return false
            }
        }
        return true;
    }
    fn check_rows(&self) -> bool {
        for row in self.rows.iter(){
            if BingoBoard::check_row(row) {
                return true;
            }
        }
        return false;
    }
    fn check_columns(&self) -> bool {
        for i in 0..4 {
            let mut completed= true;
            for row in self.rows.iter(){
                if ! row[i].marked {
                    completed = false;
                }
            }
            if completed {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::{utils};
    use crate::day4::{BingoBoard, Cell};

    #[test]
    fn part1() {
        println!("--- DAY 4-1 ----");
        let _input = utils::read_file("inputs/4.txt");
    }

    #[test]
    fn part2() {
        println!("--- DAY 4-2 ----");
        let _input = utils::read_file("inputs/4.txt");
    }

    #[test]
    fn test_finished() {

        let no_match = BingoBoard::new();
        assert!(!no_match.is_finished());
        let horizontal = BingoBoard{
            rows: [[Cell{value:0,marked:false};5],[Cell{value:0,marked:false};5],[Cell{value:0,marked:true};5],[Cell{value:0,marked:false};5],[Cell{value:0,marked:false};5]]
        };
        assert!(horizontal.is_finished());
        let vertical = BingoBoard{
            rows: [[Cell{value:0,marked:false},Cell{value:1,marked:true},Cell{value:0,marked:false},Cell{value:0,marked:false},Cell{value:0,marked:false}];5]
        };
        assert!(vertical.is_finished());

        return;
    }
}