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

    fn mark_value(&mut self, value: u32) -> bool{
        let mut marked=false;
        for row in self.rows.iter_mut() {
            for cell in row.iter_mut(){
                if cell.value == value {
                    cell.marked = true;
                    marked=true;
                }
            }
        }
        return marked;
    }
    fn board_score(&self) -> u32{
        let mut score= 0;
        for row in self.rows.iter() {
            for cell in row.iter(){
                if !cell.marked {
                    score = score + cell.value
                }
            }
        }
        return score;
    }
}

fn read_bingo_numbers (line: &String) -> Vec<i32>{
    let mut bingo_numbers = Vec::new();
    let raw_inputs=line.split(",").collect::<Vec<&str>>();
    for raw in raw_inputs.iter() {
        let z = (raw.to_string()).parse::<i32>().unwrap();
        bingo_numbers.push(z);
    }
    return bingo_numbers;
}

#[cfg(test)]
mod tests {
    use crate::{utils};
    use crate::day4::{BingoBoard, Cell, read_bingo_numbers};

    #[test]
    fn part1() {
        println!("--- DAY 4-1 ----");
        let _input = utils::read_file("inputs/4.txt");
        let line = _input.first().unwrap();
        let bingoNumbers = read_bingo_numbers(line);



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

    #[test]
    fn test_marking() {

        let mut board = BingoBoard::new();
        let marked = board.mark_value(0);
        assert!(marked);
        let marked = board.mark_value(10);
        assert!(!marked);

        return;
    }
    #[test]
    fn test_boardscore() {

        let board = BingoBoard::new();
        let score = board.board_score();
        assert_eq!(score, 0);
        let score = BingoBoard{
            rows: [[Cell{value:1,marked:false},Cell{value:1,marked:true},Cell{value:2,marked:false},Cell{value:3,marked:false},Cell{value:4,marked:false}];5]
        }.board_score();
        assert_eq!(score, 50);

        return;
    }
}