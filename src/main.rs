use std::fs::File;
use std::io::Read;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
enum Lot {
    Dot,
    Hash,
    Guard(Direction),
    Visited,
}

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

fn create_field(buff: String) -> (Vec<Vec<Lot>>, Position) {
    let mut field: Vec<Vec<Lot>> = Vec::new();
    let mut guard_position = (0, 0);

    for (y, line) in buff.lines().enumerate() {
        let mut vline: Vec<Lot> = Vec::new();
        for (x, part) in line.chars().enumerate() {
            match part {
                '.' => vline.push(Lot::Dot),
                '#' => vline.push(Lot::Hash),

                '>' => {
                    vline.push(Lot::Guard(Direction::Right));
                    guard_position = (x, y);
                }

                '^' => {
                    vline.push(Lot::Guard(Direction::Up));
                    guard_position = (x, y);
                }

                'v' => {
                    vline.push(Lot::Guard(Direction::Down));
                    guard_position = (x, y);
                }

                '<' => {
                    vline.push(Lot::Guard(Direction::Left));
                    guard_position = (x, y);
                }
                _ => panic!("should not happen"),
            }
        }
        field.push(vline);
    }
    let (x, y) = guard_position;
    (field, Position { x, y })
}

fn move_player(field: &mut Vec<Vec<Lot>>, mut guard_position: Position) -> Option<Position> {
    // guards position

    let x = guard_position.x;
    let y = guard_position.y;
    println!("move: {}, {} field: {:?}", x, y, field[y][x]);
    match field[y][x] {
        Lot::Guard(Direction::Up) => {
            if y > field.len() - 1 {
                return None;
            } else {
                field[y][x] = Lot::Visited;
                field[y + 1][x] =
                    Lot::Guard(check_collisions(field, guard_position, Direction::Up));
                guard_position = Position { x, y: y + 1 };
            }
        }

        Lot::Guard(Direction::Down) => {
            if y < field.len() - 1 {
                return None;
            } else {
                field[y][x] = Lot::Visited;
                field[y - 1][x] =
                    Lot::Guard(check_collisions(field, guard_position, Direction::Down));
                guard_position = Position { x, y: y - 1 }
            };
        }

        Lot::Guard(Direction::Right) => {
            if x > field[y].len() - 1 {
                return None;
            } else {
                field[y][x] = Lot::Visited;
                field[y][x + 1] =
                    Lot::Guard(check_collisions(field, guard_position, Direction::Right));
                guard_position = Position { x: x + 1, y }
            };
        }

        Lot::Guard(Direction::Left) => {
            if x < field[y].len() - 1 {
                return None;
            } else {
                field[y][x] = Lot::Visited;
                field[y][x - 1] =
                    Lot::Guard(check_collisions(field, guard_position, Direction::Left));
                guard_position = Position { x: x - 1, y }
            };
        }
        _ => panic!("shouldn't happen lol"),
    };
    Some(guard_position)
}

fn check_collisions(
    field: &mut Vec<Vec<Lot>>,
    guard_position: Position,
    direction: Direction,
) -> Direction {
    let x = guard_position.x;
    let y = guard_position.y;
    match direction {
        Direction::Up => {
            if y > field.len() {
                
            }
            match field[y + 1][x] {
                Lot::Hash => Direction::Right,
                _ => Direction::Up,
            }
        }

        Direction::Right => match field[y][x + 1] {
            Lot::Hash => Direction::Down,
            _ => Direction::Right,
        },

        Direction::Down => match field[y - 1][x] {
            Lot::Hash => Direction::Left,
            _ => Direction::Down,
        },

        Direction::Left => match field[y][x - 1] {
            Lot::Hash => Direction::Left,
            _ => Direction::Down,
        },
    }
}

fn main() {
    let mut result = 0;

    let mut buff = String::new();
    let mut f = File::open("input.txt").unwrap();
    f.read_to_string(&mut buff).unwrap();

    let (mut field, mut guard_position) = create_field(buff);

    // let x = guard_position.x;
    // let y = guard_position.y;

    loop {
        guard_position = match move_player(&mut field, guard_position) {
            Some(g) => {
                result += 1;
                g
            }
            None => {
                println!("{result}");
                break;
            }
        }
    }
    // println!("{field:?}");
    // println!("guard: {guard_position:?}");
    // let ac_guard = guard_position.clone();
    // guard_position = match move_player(&mut field, guard_position) {
    //     Some(g) => {
    //         result += 1;
    //         g
    //     }
    //     None => ac_guard,
    // };

    // println!("guard2: {guard_position:?}");

    // println!("{result}");
    // println!("{field:?}");
}
