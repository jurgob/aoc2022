fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", pt_1(input));
}

type Position = (i32, i32);
type Positions = (Position, Position);

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl std::str::FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            "L" => Ok(Direction::L),
            "R" => Ok(Direction::R),
            _ => Err(()),
        }
    }
}
    
fn pt_1(input: &str) -> i32 {
    let mut positions:Positions = ((0,0),(0,0)); 
    let mut tail_positions_visited: Vec<Position> = Vec::new();

    println!("initial position: {:?}", positions);
    input.lines().map(|line|{
        let (cmd, val) = line.split_once(" ").unwrap();
        (cmd.parse::<Direction>().unwrap(), val.parse().unwrap())
    }).for_each(|pos| {
        let (direction, value) = pos;
        println!("move direction: {:?}, value: {:?}", direction, value);
        for idx in 0..value {
            let (head, tail) = make_move(positions, &direction);
            positions = (head, tail);
            println!("idx: {}, position: {:?}",idx,  positions);
            if !tail_positions_visited.contains(&tail) {
                println!("- add tail_new: {:?}", tail);
                tail_positions_visited.push(tail);
            }
        }
    });
    println!("final position: {:?}", positions);
    tail_positions_visited.iter().count() as i32
}


fn make_move(positions: Positions, direction: &Direction) -> Positions {
    let (head, tail) = positions;
    // let (x1, y1) = head;
    // let (x2, y2) = tail;
    let next_head = move_direction(head, direction);
    let tail_next_head_x_distance = (next_head.0 - tail.0).abs();
    let tail_next_head_y_distance = (next_head.1 - tail.1).abs();

    if tail_next_head_x_distance <= 1  && tail_next_head_y_distance <= 1 {
        (next_head, tail)
    }else {
        (next_head, head)
    }

}

fn move_direction(p: Position, direction: &Direction) -> Position {
    let (x, y) = p;
    match direction {
        Direction::U => (x, y+1),
        Direction::D => (x, y-1),
        Direction::L => (x-1, y),
        Direction::R => (x+1, y),
    }
}

#[cfg(test)]
mod tests{
    use crate::Position;

    use super::move_direction;
    use super::make_move;

    #[test]
    fn test_move_up() {
        assert_eq!(move_direction((0,0), &crate::Direction::U), (0,1));
    }

    #[test]
    fn test_move_down() {
        assert_eq!(move_direction((0,0),&crate::Direction::D), (0,-1));
    }

    #[test]
    fn test_move_left() {
        assert_eq!(move_direction((0,0),&crate::Direction::L), (-1,0));
    }
    #[test]
    fn test_move_right() {
        assert_eq!(move_direction((0,0),&crate::Direction::R), (1,0));
    }

    #[test]
    fn test_make_move_same_position() {
        let same_position = ((0,0), (0,0));
        assert_eq!(make_move(same_position, &crate::Direction::R),((1,0), (0,0)));
        assert_eq!(make_move(same_position, &crate::Direction::L),((-1,0), (0,0)));
        assert_eq!(make_move(same_position, &crate::Direction::U),((0,1), (0,0)));
        assert_eq!(make_move(same_position, &crate::Direction::D),((0,-1), (0,0)));
    }

    #[test]
    fn test_hashmap(){
        // I'm doing this test becase I'm not sure if I can use a hashmap to store the positions

        let mut positions: Vec<Position> = Vec::new();
        let position1 = (0,0);
        let position2 = (0,0);
        positions.push(position1);
        if !positions.contains(&position2) {
            positions.push(position2);
        }
        assert_eq!(positions.iter().count(), 1)
        

    }

}
#[cfg(test)]
mod test_pt_1 {
    use super::pt_1;

    #[test]
    fn test_pt_1() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(pt_1(input), 13);
    }

    #[test]
    fn test_pt_1_1() {
        let input = "R 4";

        assert_eq!(pt_1(input), 4);
    }

    #[test]
    fn test_pt_1_2() {
        let input = "R 4
U 4";

        assert_eq!(pt_1(input), 7);
    }
}