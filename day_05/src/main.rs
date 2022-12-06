fn main() {
    println!("Hello, world!");
    let input = include_str!("../input.txt");
    let (stacks_input, moves_input) = input.split_once("\n\n").unwrap();
    
    let mut stacks_input_iter = stacks_input.lines().rev();
    
    let stacks_cols_input = stacks_input_iter.next().unwrap();
    let cols_parse:Vec<&str> = stacks_cols_input.split("   ").collect();
    let cols_len = cols_parse.len();
    
    let mut i:usize = 0;

    type Cols = Vec<Vec<char>>;
    let mut cols:Cols =Vec::new();
    let mut cols_pt2:Cols =Vec::new();

    while i < cols_len {
        cols.insert(i, Vec::new());
        cols_pt2.insert(i, Vec::new());
        i+=1;
    }

    stacks_input_iter.enumerate().for_each(|(_idx, line)| {
        // let stack_entry = parse_entry(line).clone();
        
        let line_obj  = parse_entry(line);
        line_obj.iter().enumerate().for_each(|(i, item)|{
            // let stack = cols.get_mut(i).unwrap();
            
            if *item != ' ' {
                cols.get_mut(i).unwrap().push(*item);
                cols_pt2.get_mut(i).unwrap().push(*item);
            }

        });
    });


    let moves: Vec<Move> = moves_input.lines().map(|line| {
        parse_move(line)
    }).collect();
    println!("-- PT 1 --");
    println!("cols {:?}",cols );

    moves.iter().for_each(| m | {
        let mut i = 0;
        while i< m.0 {
            let col_from = cols.get_mut(m.1).unwrap();
            let c = col_from.pop().unwrap();
            
            let col_to = cols.get_mut(m.2).unwrap();
            col_to.push(c);

            i+=1;
        }  
    });

    println!("cols {:?}",cols );

    let res: String = cols.iter().map(| col| {
        col.last().unwrap()
    }).collect();

    println!("pt1 res:  {:?}", res );
    println!("-- PT 2 --");
    println!("cols {:?}",cols_pt2 );

    moves.iter().for_each(| m | {
        
        let col_from = cols_pt2.get_mut(m.1).unwrap();
        let new_len = col_from.len() - m.0;

        let moving_stuff:Vec<char> = col_from.drain(new_len..).collect();
                
        let col_to = cols_pt2.get_mut(m.2).unwrap();
        moving_stuff.iter().for_each(|c| {
            col_to.push(*c); 
        });

       
    });

    let res_pt2: String = cols_pt2.iter().map(| col| {
        col.last().unwrap()
    }).collect();

    println!("pt2 res:  {:?}", res_pt2 );



}

type Move<'a> = (usize,usize,usize);
fn parse_move(input: &str) -> Move {
    let res:Vec<&str> = input.trim().split_ascii_whitespace().collect();
    let res:Move = (
        res.get(1).unwrap().parse().unwrap(),
        res.get(3).unwrap().parse().unwrap(),
        res.get(5).unwrap().parse().unwrap()
    );

    (res.0, res.1-1, res.2-1)
    
}

type StackEntry = Vec<char>;
fn parse_entry(input: &str) -> StackEntry {
    input.chars().skip(1).step_by(4).collect()
}

#[cfg(test)]
mod tests {
    use crate::StackEntry;
    use crate::parse_move;
    use crate::parse_entry;
    #[test]
    fn parse_input_works(){
        assert_eq!(parse_move(&"move 1 from 2 to 1"),(1,1,0));
    }

    #[test]
    fn parse_entry_works_all_entry(){
        let mut res:StackEntry = Vec::new();
        res.push('Z');
        res.push('M');
        res.push('P');
        assert_eq!(*parse_entry("[Z] [M] [P]"),res);
    }
    
    #[test]
    fn parse_entry_works_middle_entry(){
        let mut res:StackEntry = Vec::new();
        res.push(' ');
        res.push('D');
        res.push(' ');
        assert_eq!(*parse_entry("    [D]    "),res);
    }

    // fn parse_entry_works_middle_entry(){
    //     assert_eq!(parse_entry(&"    [D]    "),('-','M','-'));
    // }

}