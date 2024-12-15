use util::grid::Grid;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let (map, movements) = input.split_once("\n\n").unwrap();

    let mut start_x = 0;
    let mut start_y = 0;
    let mut grid = Grid::new(
        input.lines().next().unwrap().len() as u32,
        input.lines().count() as u32,
    );
    for (y, line) in map.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => Tile::Wall,
                'O' => Tile::Box,
                '.' => Tile::Empty,
                '@' => {
                    start_x = x as u32;
                    start_y = y as u32;
                    Tile::Empty
                }
                _ => panic!(),
            };
            grid.set(x as u32, y as u32, tile);
        }
    }

    let mut robot_x = start_x;
    let mut robot_y = start_y;
    'outer: for movement in movements.chars().filter(|c| *c != '\n') {
        let (offset_x, offset_y) = match movement {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!(),
        };
        let new_x = robot_x.wrapping_add_signed(offset_x);
        let new_y = robot_y.wrapping_add_signed(offset_y);

        match grid.get_at(new_x, new_y).unwrap() {
            Tile::Box => {
                let mut check_x = new_x.wrapping_add_signed(offset_x);
                let mut check_y = new_y.wrapping_add_signed(offset_y);
                loop {
                    match *grid.get_at(check_x, check_y).unwrap() {
                        Tile::Box => {
                            check_x = check_x.wrapping_add_signed(offset_x);
                            check_y = check_y.wrapping_add_signed(offset_y);
                        }
                        Tile::Empty => break,
                        Tile::Wall => continue 'outer,
                    }
                }

                robot_x = new_x;
                robot_y = new_y;
                grid.set(new_x, new_y, Tile::Empty);
                grid.set(check_x, check_y, Tile::Box);
            }
            Tile::Empty => {
                robot_x = new_x;
                robot_y = new_y;
            }
            Tile::Wall => (),
        }
    }

    let mut part1 = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if let Some(Tile::Box) = grid.get_at(x, y) {
                part1 += 100 * y as u64 + x as u64;
            }
        }
    }

    let mut start_x = 0;
    let mut start_y = 0;
    let mut grid = Grid::new(
        2 * map.lines().next().unwrap().len() as u32,
        map.lines().count() as u32,
    );
    for (y, line) in map.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (left, right) = match c {
                '#' => (Tile2::Wall, Tile2::Wall),
                'O' => (Tile2::Box(true), Tile2::Box(false)),
                '.' => (Tile2::Empty, Tile2::Empty),
                '@' => {
                    start_x = 2 * x as u32;
                    start_y = y as u32;
                    (Tile2::Empty, Tile2::Empty)
                }
                _ => panic!(),
            };
            grid.set(2 * x as u32, y as u32, left);
            grid.set(2 * x as u32 + 1, y as u32, right);
        }
    }

    let mut robot_x = start_x;
    let mut robot_y = start_y;
    'outer: for movement in movements.chars().filter(|c| *c != '\n') {
        let (offset_x, offset_y) = match movement {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!(),
        };
        let new_x = robot_x.wrapping_add_signed(offset_x);
        let new_y = robot_y.wrapping_add_signed(offset_y);

        match grid.get_at(new_x, new_y).unwrap() {
            Tile2::Box(_) => {
                if offset_y != 0 {
                    let mut pushed = Vec::new();
                    let mut todo = vec![(new_x, new_y)];
                    while let Some((x, y)) = todo.pop() {
                        match grid.get_at(x, y).unwrap() {
                            &Tile2::Box(left) => {
                                let (other_x, other_y) = if left { (x + 1, y) } else { (x - 1, y) };

                                pushed.push((other_x, other_y, !left));
                                todo.push((other_x, other_y.wrapping_add_signed(offset_y)));

                                pushed.push((x, y, left));
                                todo.push((x, y.wrapping_add_signed(offset_y)));
                            }
                            Tile2::Wall => continue 'outer,
                            Tile2::Empty => (),
                        }
                    }

                    for &(x, y, _) in pushed.iter() {
                        grid.set(x, y, Tile2::Empty);
                    }
                    for &(x, y, left) in pushed.iter() {
                        grid.set(x, y.wrapping_add_signed(offset_y), Tile2::Box(left));
                    }
                    robot_x = new_x;
                    robot_y = new_y;
                } else {
                    let check_y = new_y;
                    let mut check_x = new_x;
                    let mut pushed = Vec::new();
                    loop {
                        match grid.get_at(check_x, check_y).unwrap() {
                            &Tile2::Box(left) => {
                                pushed.push((check_x, check_y, left));
                                check_x = check_x.wrapping_add_signed(offset_x);
                            }
                            Tile2::Wall => continue 'outer,
                            Tile2::Empty => break,
                        }
                    }

                    grid.set(new_x, new_y, Tile2::Empty);
                    for &(x, y, left) in pushed.iter() {
                        grid.set(x.wrapping_add_signed(offset_x), y, Tile2::Box(left));
                    }
                    robot_x = new_x;
                    robot_y = new_y;
                }
            }
            Tile2::Empty => {
                robot_x = new_x;
                robot_y = new_y;
            }
            Tile2::Wall => (),
        }
    }

    let mut part2 = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if let Some(Tile2::Box(true)) = grid.get_at(x, y) {
                part2 += 100 * y as u64 + x as u64;
            }
        }
    }

    Solution::from((part1, part2))
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Tile {
    #[default]
    Empty,
    Wall,
    Box,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Tile2 {
    #[default]
    Empty,
    Wall,
    Box(bool),
}
