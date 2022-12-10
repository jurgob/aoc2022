
fn main() {
    let input = include_str!("../input.txt");
    println!("pt 1: {}", pt_1(input)); 
    println!("pt 2: {}", pt_2(input)); 

}

fn pt_1(input: &str) -> usize {
    let mut matrix_visibility: Vec<Vec<bool>> = Vec::new();
    let matrix = input_2_grid(input);
    let mut count = 0;
    for (i, row) in matrix.iter().enumerate() {
        let mut row_vec:Vec<bool> = Vec::new();
        
        for (j, _cell) in row.iter().enumerate() {
            let is_vis = is_visible((i, j), &matrix);
            row_vec.push(is_vis);
            if is_vis {
                count += 1;
            }
        }
        matrix_visibility.push(row_vec);
    }

    count
}

fn pt_2(input: &str) -> usize {
    // let mut matrix_visibility: Vec<Vec<bool>> = Vec::new();
    let matrix = input_2_grid(input);
    let mut score = 0;
    for (i, row) in matrix.iter().enumerate() {
        // let mut row_vec:Vec<bool> = Vec::new();
        
        for (j, t) in row.iter().enumerate() {
            let coords = (i, j);
            
            let up_dir = get_direction(coords, Direction::Up, &matrix);
            let down_dir = get_direction(coords, Direction::Down, &matrix);
            let left_dir = get_direction(coords, Direction::Left, &matrix);
            let right_dir = get_direction(coords, Direction::Right, &matrix);
            let cur_score = get_direction_scenic_score(*t, &up_dir)
             * get_direction_scenic_score(*t, &down_dir)
             * get_direction_scenic_score(*t, &left_dir)
             * get_direction_scenic_score(*t, &right_dir);
            if cur_score >= score {
                score = cur_score;
            }
            // let is_vis = is_visible((i, j), &matrix);
            // row_vec.push(is_vis);
            // if is_vis {
            //     count += 1;
            // }
        }
        // matrix_visibility.push(row_vec);
    }
    score
    // count
}

fn get_direction_scenic_score(t: usize, raw:&TreeRaw) -> usize {
    let mut res = 0;
    for ele in raw {
        res = res + 1;
        if &t <= ele {
            return res;
        }
    }
    return res;
}

type TreeMatrix = Vec<TreeRaw>;
type TreeRaw = Vec<usize>;
type Coords = (usize, usize);

fn input_2_grid(input: &str) -> TreeMatrix {
    input
        .lines()
        .map(|line| line.chars().map(|t| {
            t.to_string().parse::<usize>().unwrap()
        }).collect())
        .collect()
}


fn is_visible(coords:Coords, matrix:&TreeMatrix) -> bool{
   
    let visibilty_directions = is_visible_from_directions(coords, matrix);
    
    visibilty_directions.up || visibilty_directions.down || visibilty_directions.left || visibilty_directions.right
}

#[derive(Debug)]
struct VisibiltyDirections {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl PartialEq for VisibiltyDirections {
    fn eq(&self, other: &Self) -> bool {
        self.up == other.up && self.down == other.down && self.left == other.left && self.right == other.right
    }
}   


fn is_visible_from_directions(coords:Coords, matrix:&TreeMatrix) -> VisibiltyDirections {
    let width = matrix[0].len();
    let height = matrix.len();

    let tree_h = matrix[coords.0][coords.1];
    
    let vis_up = if coords.0 == 0 { true } else {
        is_visible_from_direction(tree_h, &get_direction(coords, Direction::Up, matrix)) 
    };
    
    let vis_down = if coords.0 ==  height - 1 { true } else {
        is_visible_from_direction(tree_h, &get_direction(coords, Direction::Down, matrix))
    };

    let vis_left = if coords.1 == 0 { true } else {
        is_visible_from_direction(tree_h, &get_direction(coords, Direction::Left, matrix))
    };
    let vis_right = if coords.1 ==width -1 { true } else {
            is_visible_from_direction(tree_h, &get_direction(coords, Direction::Right, matrix))
    };
    
    return VisibiltyDirections {
        up: vis_up,
        down: vis_down,
        left: vis_left,
        right: vis_right,
    };
           
}

fn is_visible_from_direction(n:usize, raw:&TreeRaw) -> bool {
    raw.iter().filter(|t| t >= &&n).count() <= 0
    
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
fn get_direction(coords:Coords, direction: Direction, matrix:&TreeMatrix) -> TreeRaw {
    match direction {
        Direction::Up => {
            let mut res:TreeRaw = vec![];
            for i in (0..coords.0).rev() {
                res.push(matrix[i][coords.1]);
            }
            res
        },
        Direction::Down => {
            let mut res:TreeRaw = vec![];
            for i in coords.0+1..matrix.len() {
                res.push(matrix[i][coords.1]);
            }
            res
        },
        Direction::Left => {
            let mut res:TreeRaw = vec![];
            for i in (0..coords.1).rev() {
                res.push(matrix[coords.0][i]);
            }
            res
        },
        Direction::Right => {
            let mut res:TreeRaw = vec![];
            for i in coords.1+1..matrix[0].len() {
                res.push(matrix[coords.0][i]);
            }
            res
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{input_2_grid, is_visible, TreeMatrix, is_visible_from_direction, get_direction, Direction, pt_1, is_visible_from_directions, VisibiltyDirections, get_direction_scenic_score, pt_2};

    #[test]
    fn get_direction_scenic_score_work(){
        assert_eq!(get_direction_scenic_score(5, &vec![3,5,3]), 2, "second 5 up");
        assert_eq!(get_direction_scenic_score(5, &vec![3,3]), 2, "second 5 left");
        assert_eq!(get_direction_scenic_score(5, &vec![3]), 1, "second 5 down");
        assert_eq!(get_direction_scenic_score(5, &vec![4,9]), 2, "second 5 right");
    }

    #[test]
    fn test_input_2_grid() {
        let input = "30373
25512
65332
33549
35390";

        let res = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]; 

        assert_eq!(input_2_grid(input).get(0), res.get(0), "first row of matrix is not what we expected");
        assert_eq!(input_2_grid(input), res, "matrix is not what we expected")
    }

    #[test]
    fn test_is_visible() {
        let res:TreeMatrix = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]; 

        assert_eq!(is_visible((2,2), &res), false, "middle 3 is not visible");
        assert_eq!(is_visible((2,1), &res), true, "middle left 5 is visible");
        assert_eq!(is_visible((0,0), &res), true, "first cell  is visible");
        assert_eq!(is_visible((0,0), &res), true, "last cell first raw is visible");
        assert_eq!(is_visible((1,4), &res), true, "last cell second raw is visible");
        assert_eq!(is_visible((4,4), &res), true, "last cell last raw is visible");
        assert_eq!(is_visible((1,3), &res), false, "top right 1 is not visible");
        assert_eq!(is_visible((3,2), &res), true, "bottom right 5 is  visible");
        assert_eq!(is_visible((3,3), &res), false, "bottom right 4 is not visible");
        
    }
    
    
    #[test]
    fn is_visible_from_direction_works(){
        assert_eq!(is_visible_from_direction(5, &vec![3]), true);
        assert_eq!(is_visible_from_direction(5, &vec![3,4,1]), true);
        assert_eq!(is_visible_from_direction(5, &vec![]), true);
        assert_eq!(is_visible_from_direction(5, &vec![3,1,3,4,0]), true);

        assert_eq!(is_visible_from_direction(5, &vec![7,4,1]), false);
        assert_eq!(is_visible_from_direction(5, &vec![5]), false);
        assert_eq!(is_visible_from_direction(5, &vec![7]), false);
        assert_eq!(is_visible_from_direction(5, &vec![1,3,7]), false);
        assert_eq!(is_visible_from_direction(5, &vec![1,7,3]), false);
        assert_eq!(is_visible_from_direction(3, &vec![3,2]), false);
        
    }


    #[test]
    fn get_direction_works(){
        let tree_matrix:TreeMatrix = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3/*here*/, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]; 
        let coords = (2,3); //3
        assert_eq!(get_direction(coords, Direction::Up, &tree_matrix), vec![1,7], "up is what what we expected");
        assert_eq!(get_direction(coords, Direction::Down, &tree_matrix), vec![4,9], "down is what what we expected");
        assert_eq!(get_direction(coords, Direction::Left, &tree_matrix), vec![3,5,6], "left is what what we expected");
        assert_eq!(get_direction(coords, Direction::Right, &tree_matrix), vec![2], "right is what what we expected");
        
    }

    #[test]
    fn test_is_visible_from_directions() {
        
        assert_eq!(
            is_visible_from_directions( 
                (1,1) ,
                &vec![
                    vec![0,0,0],
                    vec![0,1,0],
                    vec![0,0,0],
                ],
            ), 
            VisibiltyDirections{up:true, down:true,left:true,right:true},
            "visible in any direction, 3x3 matrix"
        
        );
        
    }

    #[test]
    fn pt_1_works(){
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(pt_1(input), 21);
    }
    
    #[test]
    fn pt_2_works(){
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(pt_2(input), 8);
    }

    

}