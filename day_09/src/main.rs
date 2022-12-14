fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", pt_1(input));
    println!("Part 2: {}", pt_2(input));
}

type Position = (i32, i32);
type Positions = (Position, Position);

//create a type of N positions using an array
type Rope10 = [Position; 10];




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

    // println!("initial position: {:?}", positions);
    input.lines().map(|line|{
        let (cmd, val) = line.split_once(" ").unwrap();
        (cmd.parse::<Direction>().unwrap(), val.parse().unwrap())
    }).for_each(|pos| {
        let (direction, value) = pos;
        // println!("move direction: {:?}, value: {:?}", direction, value);
        for _idx in 0..value {
            let (head, tail) = make_move(positions, &direction);
            positions = (head, tail);
            // println!("idx: {}, position: {:?}",_idx,  positions);
            if !tail_positions_visited.contains(&tail) {
                // println!("- add tail_new: {:?}", tail);
                tail_positions_visited.push(tail);
            }
        }
    });
    println!("final position: {:?}", positions);
    tail_positions_visited.iter().count() as i32
}

fn pt_2(input: &str) -> i32 {
    let mut cur_rope_position:Rope10 = [(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)]; 
    let mut positions:Positions = ((0,0),(0,0)); 
    let mut tail_positions_visited: Vec<Position> = Vec::new();
    let log = false;

    if log { println!("initial position: {:?}", positions); }
    input.lines().map(|line|{
        let (cmd, val) = line.split_once(" ").unwrap();
        (cmd.parse::<Direction>().unwrap(), val.parse().unwrap())
    }).for_each(|pos| {
        let (direction, value) = pos;
        if log { println!("move direction: {:?}, value: {:?}", direction, value); }
        for _idx in 0..value {
            let (head, tail) = make_move_rope(&mut cur_rope_position, &direction);
            positions = (head, tail);
            if log { 
                println!("idx: {}, position: {:?}",_idx,  positions);
                println!("cur_rope_position: {:?}",cur_rope_position);
            }
            if !tail_positions_visited.contains(&tail) {
                if log { println!("- add tail_new: {:?}", tail); }
                tail_positions_visited.push(tail);
            }
        }
    });
    if log { println!("final position: {:?}", positions); }
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

fn is_node_too_detach(head: Position, tail: Position) -> bool {
    let tail_next_head_x_distance = (head.0 - tail.0).abs();
    let tail_next_head_y_distance = (head.1 - tail.1).abs();
    tail_next_head_x_distance > 1  || tail_next_head_y_distance > 1
}

fn make_move_rope(rope: &mut Rope10, direction: &Direction) -> Positions {
    // let mut head = rope[0];
    // let mut tail = rope[9];
    let rope_len = rope.len();
    let mut rope_new = rope.clone();
    for idx in 0..rope_len - 1 {
        let h = rope[idx];
        let t = rope[idx+1];
        let (h_next, t_next) = make_move((h,t), direction);
        rope_new[idx] = h_next;
        rope_new[idx+1] = t_next;
        if idx < rope_len - 3 {
            let tt = rope[idx+2];
            let move_next_knot = is_node_too_detach(t_next, tt);
            println!("tt {:?}, t_next: {:?}, move_next_knot: {}", tt, t_next, move_next_knot);
            if !move_next_knot {
                break;
            }
        }
    }
    
    for idx in 0..rope_len - 1 {
        rope[idx] = rope_new[idx];
    }
    
    (rope_new[0], rope_new[rope_len -1])
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
    use super::make_move_rope;
    use super::is_node_too_detach;

    #[test]
    fn test_is_node_too_detach_true_1() {
        let head = (0,0);
        let tail = (2,0);
        assert_eq!(is_node_too_detach(head, tail), true);
    }
    
    #[test]
    fn test_is_node_too_detach_true_2() {
        let head = (0,0);
        let tail = (0,2);
        assert_eq!(is_node_too_detach(head, tail), true);
    }

    #[test]
    fn test_is_node_too_detach_false_1() {
        let head = (0,0);
        let tail = (0,1);
        assert_eq!(is_node_too_detach(head, tail), false);
    }

    #[test]
    fn test_is_node_too_detach_false_2() {
        let head = (0,0);
        let tail = (1,0);
        assert_eq!(is_node_too_detach(head, tail), false);
    }

    #[test]
    fn test_is_node_too_detach_false_same_coords() {
        let head = (0,0);
        let tail = (0,0);
        assert_eq!(is_node_too_detach(head, tail), false);
    }

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

    #[test]
    fn test_make_move_rope_r1() {
        let mut rope = [(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)];
        let rope_new = make_move_rope(&mut rope, &crate::Direction::R);
        assert_eq!(rope_new, ((1,0),(0,0)));
        assert_eq!(rope, [(1,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)]);
    }

    #[test]
    fn test_make_move_rope_r2() {
        let mut rope = [(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)];
        let rope_new = make_move_rope(&mut rope, &crate::Direction::R);
        let rope_new_2 = make_move_rope(&mut rope, &crate::Direction::R);
        assert_eq!(rope_new, ((1,0),(0,0)));
        assert_eq!(rope_new_2, ((2,0),(0,0)));
        assert_eq!(rope, [(2,0),(1,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)]);
    }


    #[test]
    fn test_make_move_rope_r3() {
        let mut rope = [(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)];
        make_move_rope(&mut rope, &crate::Direction::R);
        make_move_rope(&mut rope, &crate::Direction::R);
        let rope_new_3 = make_move_rope(&mut rope, &crate::Direction::R);
        assert_eq!(rope_new_3, ((3,0),(0,0)));
        assert_eq!(rope, [(3,0),(2,0),(1,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)]);
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

#[cfg(test)]
mod test_pt_2 {
    use super::pt_2;

    #[test]
    fn test_pt_2() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        assert_eq!(pt_2(input), 36);
    }

    #[test]
    fn test_pt_2_r5_u8() {
        let input = "R 5
U 8
";

        assert_eq!(pt_2(input), 36);
    }

    #[test]
    fn test_pt_2_r1_() {
        let input = "R 1";

        assert_eq!(pt_2(input), 1);
    }
    
    #[test]
    fn test_pt_2_r9() {
        let input = "R 9";

        assert_eq!(pt_2(input), 1);
    }

    #[test]
    fn test_pt_2_r11() {
        let input = "R 11";

        assert_eq!(pt_2(input), 3);
    }
    
}