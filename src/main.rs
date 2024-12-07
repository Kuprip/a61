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

fn create_field(
    buff: String
) -> (Vec<Vec<Lot>>, (usize, usize)) {

    let mut field: Vec<Vec<Lot>> = Vec::new();
    let mut guard_position = (0, 0);

    for (x, line) in buff.lines().enumerate() {
        let mut vline: Vec<Lot> = Vec::new();
        for (y, part) in line.chars().enumerate() {
            match part {
                '.' => vline.push(Lot::Dot),
                '#' => vline.push(Lot::Hash),
                '>' => {
                    vline.push(Lot::Guard(Direction::Right, ));
                    guard_position = (x, y);
                },
                '^' => vline.push(Lot::Guard(Direction::Up, )),
                'v' => vline.push(Lot::Guard(Direction::Down, )),
                '<' => vline.push(Lot::Guard(Direction::Left, )),
                _ => panic!("should not happen"),
            }
        }
        field.push(vline);

    }
    (field, guard_position)

}

fn move_player(

    field: &mut Vec<Vec<Lot>>,
    guard_position: (usize, usize),

) {

    let (guard_x, guard_y) = guard_position;
    match field[guard_x][guard_y] {

        Lot::Guard(Direction::Up) => { 
            field[guard_x][guard_y] = Lot::Visited;
            field[guard_x][guard_y+1] = Lot::Guard(Direction::Up,);
        },

        Lot::Guard(Direction::Down) => { 
            field[guard_x][guard_y] = Lot::Visited;
            field[guard_x][guard_y-1] = Lot::Guard(Direction::Down,);
        },

        Lot::Guard(Direction::Right) => { 
            field[guard_x][guard_y] = Lot::Visited;
            field[guard_x+1][guard_y] = Lot::Guard(Direction::Right,);
        },

        Lot::Guard(Direction::Left) => { 
            field[guard_x][guard_y] = Lot::Visited;
            field[guard_x-1][guard_y] = Lot::Guard(Direction::Left,);
        },
        _ => panic!("shouldn't happen lol"),

    };
}

fn check_collisions(
    field: &mut Vec<Vec<Lot>>,
    guard_position: (usize, usize),
    direction: Direction,
) {
    
    let (guardx, guardy) = guard_position;

    let new_direction = match direction {

        Direction::Up => match field[guardx][guardy+1]  {
            Lot::Hash => Direction::Right,
            _ => Direction::Up,
        }

        Direction::Right => match field[guardx+1][guardy]  {
            Lot::Hash => Direction::Down,
            _ => Direction::Right,
        }

        Direction::Down => match field[guardx][guardy-1]  {
            Lot::Hash => Direction::Left,
            _ => Direction::Down,
        }
        
        Direction::Left => match field[guardx-1][guardy]  {
            Lot::Hash => Direction::Left,
            _ => Direction::Down,
        }

    };
    
    field[guardy][guardy] = Lot::Guard(new_direction);

}

fn main() {
    let result = 0;

    let mut buff = String::new();
    let mut f = File::open("input.txt").unwrap();
    f.read_to_string(&mut buff).unwrap();

    let (
        mut field, mut guard_position
    ) = create_field(buff);
   
    loop {
        
    }

}