//use crate::{utils};

#[derive(Copy, Clone,Debug)]
struct Cell{
    value: i32,
    marked: bool,
}

#[derive(Copy, Clone)]
struct BingoBoard{
    rows: [[Cell;5];5]
}

// impl fmt::Display for BingoBoard {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Use `self.number` to refer to each positional data point.
//         for row in self.rows.iter() {
//             write!(f, "[");
//             for cell in row.iter(){
//                 write!(f, "{} ", cell.value);
//             }
//             write!(f, "]\n");
//         }
//     }
// }


impl BingoBoard {
    fn new() -> BingoBoard{
        return BingoBoard{
            rows: [[Cell{value:404,marked:false};5];5]
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

    fn mark_value(&mut self, value: i32) -> bool{
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
    fn board_score(&self) -> i32{
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

fn read_row_numbers (line: &String) -> Vec<i32>{
    let mut numbers = Vec::new();
    let raw_inputs=line.split_whitespace().collect::<Vec<&str>>();
    for raw in raw_inputs.iter() {
        let z = (raw.to_string()).parse::<i32>().unwrap();
        numbers.push(z);
    }
    return numbers;
}

fn setup_boards (lines: Vec<String>) -> Vec<BingoBoard>{
    let mut boards = Vec::new();

    let mut board = BingoBoard::new();
    let mut row_number=0;
    for i in 1..lines.len() {
        let line=lines.get(i).unwrap();
        if line.is_empty(){
            // simly accept that one board is unusable
            boards.push(board);
            board=BingoBoard::new();
            row_number=0;
        } else{
            let numbers = read_row_numbers(line);
            assert_eq!(numbers.len(),board.rows[row_number].len());
            for i in 0..(numbers.len()) {
                board.rows[row_number][i].value=numbers.get(i).unwrap().clone();
            }
            row_number= row_number + 1;
        }
    }
    boards.push(board);
    return boards;
}

fn play_game(bingo_numbers: Vec<i32>,boards: Vec<BingoBoard>) -> (i32, i32){
    let mut myboards=boards.clone();
    for number in bingo_numbers {
        for board in myboards.iter_mut() {
            board.mark_value(number);
            if board.is_finished() {
                //println!("{}",board);
                return (number,board.board_score());
            }
        }

    }
    // Better errorcode
    panic!("Should not get here");
    // return (0,BingoBoard::new());
}

#[cfg(test)]
mod tests {
    use crate::{utils};
    use crate::day4::{BingoBoard, Cell, play_game, read_bingo_numbers, setup_boards};

    #[test]
    fn part1() {
        println!("--- DAY 4-1 ----");
        println!("Lets Play Bingo!");
        let _input = utils::read_file("inputs/4.txt");
        let line = _input.first().unwrap();
        let bingo_numbers = read_bingo_numbers(line);
        let boards = setup_boards(_input);
        let (number,board)=play_game(bingo_numbers,boards);
        let board_score=board;
        let game_score=board_score*number;
        println!("Finalscore {} * {} = {}",number,board_score,game_score);

        // Wronganswers
        assert_ne!(game_score,134505);
        // To low
        //assert_ne!(game_score,42630);



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
        let marked = board.mark_value(404);
        assert!(marked);
        let marked = board.mark_value(10);
        assert!(!marked);

        return;
    }
    #[test]
    fn test_boardscore() {

        let board = BingoBoard::new();
        let score = board.board_score();
        assert_eq!(score, 25*404);
        let score = BingoBoard{
            rows: [[Cell{value:1,marked:false},Cell{value:1,marked:true},Cell{value:2,marked:false},Cell{value:3,marked:false},Cell{value:4,marked:false}];5]
        }.board_score();
        assert_eq!(score, 50);

        return;
    }
}