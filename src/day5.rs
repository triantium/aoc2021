use crate::utils;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }
    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        if self.is_vertical() {
            let x = self.a.x;
            if self.a.y <= self.b.y {
                for y in self.a.y..=self.b.y {
                    points.push(Point { x, y });
                }
            } else {
                for y in self.b.y..=self.a.y {
                    points.push(Point { x, y });
                }
            }
        } else if self.is_horizontal() {
            let y = self.a.y;
            if self.a.x <= self.b.x {
                for x in self.a.x..=self.b.x {
                    points.push(Point { x, y });
                }
            } else {
                for x in self.b.x..=self.a.x {
                    points.push(Point { x, y });
                }
            }
        } else {
            //just diagonal
            let mut x = self.a.x;
            let mut y = self.a.y;
            while x != self.b.x && y != self.b.y {
                points.push(Point { x, y });
                if x < self.b.x {
                    x += 1;
                }
                if x > self.b.x {
                    x -= 1;
                }
                if y < self.b.y {
                    y += 1;
                }
                if y > self.b.y {
                    y -= 1;
                }
            }
            points.push(Point { x, y });
        }

        assert!(!points.is_empty());
        return points;
    }
}

#[cfg(test)]
mod tests {
    //use crate::{utils};
    use crate::day5::get_result_1;
    use crate::day5::get_result_2;

    #[test]
    fn part1() {
        println!("--- DAY 5-1 ----");
        let result_test = get_result_1("inputs/5_test.txt");
        assert_eq!(result_test, 5);
        let result = get_result_1("inputs/5.txt");
        println!("Hot Spots :{} ", result);
        assert_eq!(result, 6856)
    }

    #[test]
    fn part2() {
        println!("--- DAY 5-2 ----");
        let result_test = get_result_2("inputs/5_test.txt");
        assert_eq!(result_test, 12);
        let result = get_result_2("inputs/5.txt");
        println!("Hot Spots :{} ", result);
        assert_eq!(result, 20666)
    }
}

fn get_result_1(file: &str) -> usize {
    let input = utils::read_file(file);
    let lines: Vec<Line> = read_lines(input);

    let filtered_lines = lines
        .iter()
        .filter(|s| s.is_horizontal() || s.is_vertical())
        .map(|s| s.to_owned())
        .collect::<Vec<Line>>();

    let map = draw_lines(filtered_lines);
    let mut values = Vec::new();
    for row in map.iter() {
        for cell in row.iter() {
            if cell.to_owned() >= 2 {
                values.push(cell);
            }
        }
    }

    return values.len();
}

fn get_result_2(file: &str) -> usize {
    let input = utils::read_file(file);
    let lines: Vec<Line> = read_lines(input);

    let map = draw_lines(lines);
    let mut values = Vec::new();
    for row in map.iter() {
        for cell in row.iter() {
            if cell.to_owned() >= 2 {
                values.push(cell);
            }
        }
    }

    return values.len();
}

fn _print_map(map: Vec<Vec<i32>>) {
    for row in map.iter() {
        println!();
        for cell in row.iter() {
            print!("{}", cell);
        }
    }
    println!();
}

fn read_lines(input: Vec<String>) -> Vec<Line> {
    let mut lines = Vec::new();
    for input_line in input.iter() {
        let raw_inputs = input_line
            .split(" -> ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let point_a = raw_inputs
            .get(0)
            .unwrap()
            .split(",")
            .map(|s| s.to_string())
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let point_b = raw_inputs
            .get(1)
            .unwrap()
            .split(",")
            .map(|s| s.to_string())
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let line = Line {
            a: Point {
                x: point_a.get(0).unwrap().to_owned(),
                y: point_a.get(1).unwrap().to_owned(),
            },
            b: Point {
                x: point_b.get(0).unwrap().to_owned(),
                y: point_b.get(1).unwrap().to_owned(),
            },
        };
        lines.push(line);
    }
    return lines;
}

fn draw_lines(lines: Vec<Line>) -> Vec<Vec<i32>> {
    let mut map = Vec::new();
    for _i in 0..1000 {
        let mut line = Vec::new();
        for _j in 0..1000 {
            line.push(0);
        }
        map.push(line);
    }
    for line in lines {
        for point in line.points() {
            let current = map
                .get_mut(point.y as usize)
                .unwrap()
                .get_mut(point.x as usize)
                .unwrap();
            *current += 1;
        }
    }

    return map;
}
