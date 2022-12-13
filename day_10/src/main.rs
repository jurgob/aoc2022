use std::str::FromStr;

fn main() { 
    let input = include_str!("../input.txt");
    println!("part 1: {}", pt_1(input));
    println!("part 2:");
    println!("{}", pt_2(input));
}

fn exex_cycles(input: &str) -> Vec<i32> {
    let initial_x = 1;
    let mut cycles:Vec<i32> = vec![initial_x, initial_x];
    input.lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .for_each(|instruction| match instruction {
            Instruction::Noop => {
                let cur_x = cycles.get(cycles.len() - 1).unwrap().clone();

                cycles.push(cur_x);
            },
            Instruction::Addx(x) => {
                let cur_x = cycles.get(cycles.len() - 1).unwrap().clone();
                cycles.push(cur_x);  
                let new_x = cur_x + x;
                // println!("at cycle {} - change x value from {} to {}. x is: {}",cycles.len() , cur_x, new_x, x);
                cycles.push(new_x);
                

            },
        });
        cycles
}

fn pt_1(input: &str) -> i32 {

    let cycles:Vec<i32> = exex_cycles(input);
        
    let indexes: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let res:i32 = indexes.iter().map(|i| {
        let temp_res = i * cycles.get(*i as usize).unwrap();
        // println!("{} * {} = {}", i, cycles.get(*i as usize).unwrap(), temp_res);
        temp_res
    }).sum();

    res

}

fn pt_2(input: &str) -> String {
    let cycles:Vec<i32> = exex_cycles(input);
    let mut res:Vec<Vec<char>> = Vec::new();
    for i in 0..6 {
        let mut temp:Vec<char> = Vec::new();
        for j in 0..40 {
            let cycle_cur = 40*(i)+j+1;
            let cycle_val = cycles.get(cycle_cur as usize).unwrap();

            let char_to_add = if j >= *cycle_val-1 && j <= *cycle_val+1  {
                '#'
            } else {
                '.'
            };
            temp.push(char_to_add);
        }
        res.push(temp);
    }

    res.iter().for_each(|l| {
        let line = l.iter().collect::<String>();
    });

    let res = res.iter().map(|l| {
        let line = l.iter().collect::<String>();
        line
    }).collect::<Vec<String>>().join("\n");

    // println!("{:?}", res);  
    
    res
}

enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let op = parts.next().unwrap();
        

        match op {
            "noop" => Ok(Instruction::Noop),
            "addx" => {
                let arg = parts.next().unwrap().parse::<i32>().unwrap();
                Ok(Instruction::Addx(arg))
            },
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::pt_1;
    use super::pt_2;


    #[test]
    fn pt_1_test() {
        let input = include_str!("../input-test.txt");

        assert_eq!(pt_1(input), 13140);
    }

    #[test]
    fn pt_2_test() {
        let input = include_str!("../input-test.txt");
        let res = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(pt_2(input), res);
    }
}
