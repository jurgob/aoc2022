use std::str::FromStr;

fn main() { 
    let input = include_str!("../input.txt");
    println!("part 1: {}", pt_1(input));
}


fn pt_1(input: &str) -> i32 {
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
                println!("at cycle {} - change x value from {} to {}. x is: {}",cycles.len() , cur_x, new_x, x);
                cycles.push(new_x);
                

            },
        });
        
        let indexes: [i32; 6] = [20, 60, 100, 140, 180, 220];
        let res:i32 = indexes.iter().map(|i| {
            let temp_res = i * cycles.get(*i as usize).unwrap();
            println!("{} * {} = {}", i, cycles.get(*i as usize).unwrap(), temp_res);
            temp_res
        }).sum();

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


    #[test]
    fn pt_1_test() {
        let input = include_str!("../input-test.txt");

        assert_eq!(pt_1(input), 13140);
    }
}
