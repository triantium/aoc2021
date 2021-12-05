
use crate::{utils};

#[derive(Copy, Clone,Debug)]
struct Point{
    x: i32,
    y: i32,
}

#[derive(Copy, Clone,Debug)]
struct Line{
    a: Point,
    b: Point,
}


impl Line {

    fn is_horizontal (&self) -> bool{
        self.a.y == self.b.y
    }
    fn is_vertical (&self) -> bool{
        self.a.x == self.b.x
    }


}



#[cfg(test)]
mod tests {
    use crate::{utils};
    use crate::day5::get_result_1;

    #[test]
    fn part1() {
        println!("--- DAY 5-1 ----");
        let result_test=get_result_1("inputs/5_test.txt");
        assert_eq!(result_test,5);
        let result_test=get_result_1("inputs/5.txt");

    }

    #[test]
    fn part2() {
        println!("--- DAY 4-2 ----");
        //let input = utils::read_file("inputs/4.txt");
    }
}

fn get_result_1 (file: &str) -> usize{
    let input = utils::read_file(file);
    let lines: Vec<Line> = read_lines(input);

    let filtered_lines = lines.iter()
        .filter(|s|s.is_horizontal() || s.is_vertical())
        .map(|s|s.to_owned())
        .collect::<Vec<Line>>();

    let map = draw_lines(filtered_lines);
    let mut values = Vec::new();
    for row in map.iter(){
        println!();
        for cell in row.iter() {
            print!("{}",cell);
            if cell.to_owned() >= 2 {
                values.push(cell);
            }
        }
    }





    return values.len();
}

fn read_lines (input: Vec<String>) -> Vec<Line>{
    let mut lines = Vec::new();
    for input_line in input.iter(){
        let raw_inputs=input_line
            .split(" -> ")
            .map(|s|s.to_string())
            .collect::<Vec<String>>();
        let point_a = raw_inputs.get(0).unwrap()
            .split(",")
            .map(|s|s.to_string())
            .map(|s|s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let point_b = raw_inputs.get(1).unwrap()
            .split(",")
            .map(|s|s.to_string())
            .map(|s|s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let line = Line{
            a: Point{
                x: point_a.get(0).unwrap().to_owned(),
                y: point_a.get(1).unwrap().to_owned(),
            },
            b: Point{
                x: point_b.get(0).unwrap().to_owned(),
                y: point_b.get(1).unwrap().to_owned(),
            }
        };
        lines.push(line);

    }
    return lines;
}

fn draw_lines (lines: Vec<Line>) -> Vec<Vec<i32>>{
    let mut map = Vec::new();
    for _i in 0..1000 {
        let mut line = Vec::new();
        for _j in 0..1000 {
            line.push(0);
        }
        map.push(line);
    }
    for line in lines{
        for x in line.a.x..=line.b.x {
            for y in line.a.y..=line.b.y {
                let mut current =map.get_mut(x as usize).unwrap().get_mut(y as usize).unwrap();
                *current += 1;
                //map.get_mut(x as usize).unwrap().get_mut(y as usize).unwrap() = current;
            }
        }
    }

    return map;
}