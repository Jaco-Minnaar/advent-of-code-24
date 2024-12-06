use core::panic;
use std::collections::HashSet;
use std::io::BufRead;

use advent_of_code::get_input;

fn main() {
    let input = get_input(6);

    let map = Map::from_lines(input.lines().map_while(Result::ok));

    path_count(map.clone());
    loop_count(map);
}

fn path_count(mut map: Map) {
    let count = map.sim_path().expect("loop dectected").visited();
    println!("{count}");
}

fn loop_count(mut map: Map) {
    let loops = map.find_loops();
    println!("{loops}");
}

#[derive(Debug)]
enum SimError {
    LoopDetected,
}

#[derive(Clone)]
enum Tile {
    Open,
    Visited(HashSet<Direction>),
    Obstacle,
    Guard(Direction, Box<Tile>),
}

impl Tile {
    fn visited(&self) -> bool {
        matches!(self, Self::Visited(_))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Open,
            '#' => Tile::Obstacle,
            '^' => Tile::Guard(Direction::Up, Box::new(Tile::Open)),
            '>' => Tile::Guard(Direction::Right, Box::new(Tile::Open)),
            'V' => Tile::Guard(Direction::Down, Box::new(Tile::Open)),
            '<' => Tile::Guard(Direction::Left, Box::new(Tile::Open)),
            _ => unreachable!(),
        }
    }
}

impl From<Tile> for char {
    fn from(val: Tile) -> Self {
        match val {
            Tile::Open => '.',
            Tile::Visited(_) => 'X',
            Tile::Obstacle => '#',
            Tile::Guard(Direction::Up, _) => '^',
            Tile::Guard(Direction::Right, _) => '>',
            Tile::Guard(Direction::Down, _) => 'V',
            Tile::Guard(Direction::Left, _) => '<',
        }
    }
}

impl From<&Tile> for char {
    fn from(val: &Tile) -> Self {
        match val {
            Tile::Open => '.',
            Tile::Visited(_) => 'X',
            Tile::Obstacle => '#',
            Tile::Guard(Direction::Up, _) => '^',
            Tile::Guard(Direction::Right, _) => '>',
            Tile::Guard(Direction::Down, _) => 'V',
            Tile::Guard(Direction::Left, _) => '<',
        }
    }
}

impl Tile {
    fn guard_direction(&self) -> Option<&Direction> {
        if let Self::Guard(direction, _) = self {
            Some(direction)
        } else {
            None
        }
    }
}

enum Axis {
    Horizontal(usize),
    Vertical(usize),
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    print: bool,
}

impl Map {
    fn from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Self {
        let mut tiles = Vec::new();
        let mut h = 0;
        let mut w = None;
        for line in lines {
            let line = line.as_ref();
            h += 1;

            if w.is_none() {
                w = Some(line.len());
            }

            let mut row = Vec::new();
            for c in line.chars() {
                let tile: Tile = c.into();

                row.push(tile);
            }

            tiles.push(row);
        }

        Self {
            tiles,
            width: w.unwrap(),
            height: h,
            print: false,
        }
    }

    fn print(&mut self) -> &mut Self {
        self.print = true;
        self
    }

    fn sim_path(&mut self) -> Result<&mut Self, SimError> {
        let Some((mut gr, mut gc)) = self.guard_pos() else {
            println!("===========  ERROR  ===========\nInvalid board:");
            self.print();
            self.print_map();
            panic!("No guard on map");
        };

        loop {
            self.print_map();
            let ((lgr, lgc), encountered_obstacle, direction) = {
                let direction = *self.tiles[gr][gc]
                    .guard_direction()
                    .expect("Guard not in expected position");

                let ((lgr, lgc), encountered_obstacle) = match direction {
                    Direction::Up => {
                        self.last_pos((0..=gr).rev(), Axis::Vertical(gc), direction)?
                    }
                    Direction::Down => {
                        self.last_pos(gr..self.height, Axis::Vertical(gc), direction)?
                    }
                    Direction::Left => {
                        self.last_pos((0..=gc).rev(), Axis::Horizontal(gr), direction)?
                    }
                    Direction::Right => {
                        self.last_pos(gc..self.width, Axis::Horizontal(gr), direction)?
                    }
                };

                ((lgr, lgc), encountered_obstacle, direction)
            };

            gc = lgc;
            gr = lgr;

            if !encountered_obstacle {
                break;
            }

            let old_tile = self.tiles[gr][gc].clone();
            self.tiles[gr][gc] = Tile::Guard(direction.turn(), Box::new(old_tile));
        }

        if let Tile::Visited(_) = self.tiles[gr][gc] {
        } else {
            self.tiles[gr][gc] = Tile::Visited(HashSet::new());
        }

        self.print_map();
        Ok(self)
    }

    fn print_map(&self) {
        if !self.print {
            return;
        }

        let mut map_str = String::with_capacity((self.width + 1) * self.height);

        for row in &self.tiles {
            for tile in row {
                map_str.push(tile.into());
            }
            map_str.push('\n');
        }

        println!();
        println!("{map_str}");
    }

    fn find_loops(&mut self) -> usize {
        let start = self.guard_pos().expect("No guard on map");
        let mut loops = 0;
        let mut base = self.clone();

        base.sim_path().expect("Found loop in base map");

        let mut tested = 0;
        for r in 0..self.width {
            for c in 0..self.height {
                if (r, c) == start {
                    continue;
                }

                let tile = &base.tiles[r][c];

                if let Tile::Visited(_) = tile {
                    let mut testee = self.clone();
                    testee.tiles[r][c] = Tile::Obstacle;
                    //if tested == 496 {
                    //    testee.print();
                    //}
                    if testee.sim_path().is_err() {
                        loops += 1;
                        println!("{tested}: true");
                    } else {
                        println!("{tested}: false");
                    }

                    tested += 1;
                }
            }
        }

        loops
    }

    fn visited(&self) -> usize {
        self.tiles
            .iter()
            .map(|row| row.iter().filter(|tile| tile.visited()).count())
            .sum()
    }

    fn last_pos(
        &mut self,
        path: impl Iterator<Item = usize>,
        axis: Axis,
        direction: Direction,
    ) -> Result<((usize, usize), bool), SimError> {
        match axis {
            Axis::Horizontal(gr) => {
                let positive = direction == Direction::Right;
                for c in path {
                    let tile = &mut self.tiles[gr][c];

                    match tile {
                        Tile::Obstacle => {
                            let new_c = if positive { c - 1 } else { c + 1 };
                            return Ok(((gr, new_c), true));
                        }
                        Tile::Visited(dirs) => {
                            if dirs.contains(&direction) {
                                return Err(SimError::LoopDetected);
                            } else {
                                dirs.insert(direction);
                            }
                        }
                        Tile::Guard(_, tile) => {
                            self.tiles[gr][c] = if let Tile::Visited(dirs) = tile.as_mut() {
                                dirs.insert(direction);
                                *tile.clone()
                            } else {
                                Tile::Visited(HashSet::from([direction]))
                            }
                        }
                        _ => {
                            self.tiles[gr][c] = Tile::Visited(HashSet::from([direction]));
                        }
                    }
                }

                if positive {
                    Ok(((gr, self.width - 1), false))
                } else {
                    Ok(((gr, 0), false))
                }
            }
            Axis::Vertical(gc) => {
                let positive = direction == Direction::Down;
                for r in path {
                    let tile = &mut self.tiles[r][gc];

                    match tile {
                        Tile::Obstacle => {
                            let new_r = if positive { r - 1 } else { r + 1 };
                            return Ok(((new_r, gc), true));
                        }
                        Tile::Visited(dirs) => {
                            if dirs.contains(&direction) {
                                return Err(SimError::LoopDetected);
                            } else {
                                dirs.insert(direction);
                            }
                        }
                        Tile::Guard(_, tile) => {
                            self.tiles[r][gc] = if let Tile::Visited(dirs) = tile.as_mut() {
                                dirs.insert(direction);
                                *tile.clone()
                            } else {
                                Tile::Visited(HashSet::from([direction]))
                            }
                        }
                        _ => {
                            self.tiles[r][gc] = Tile::Visited(HashSet::from([direction]));
                        }
                    }
                }

                if positive {
                    Ok(((self.height - 1, gc), false))
                } else {
                    Ok(((0, gc), false))
                }
            }
        }
    }

    fn guard_pos(&self) -> Option<(usize, usize)> {
        for (r, row) in self.tiles.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if let Tile::Guard(_, _) = tile {
                    return Some((r, c));
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use crate::Map;

    #[test]
    fn count_visited_test_data() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let expected = 41;
        let actual = Map::from_lines(input.lines())
            .print()
            .sim_path()
            .expect("Found loop")
            .visited();

        assert_eq!(actual, expected);
    }

    #[test]
    fn count_loops() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let expected = 6;
        let actual = Map::from_lines(input.lines()).print().find_loops();

        assert_eq!(actual, expected);
    }
}
