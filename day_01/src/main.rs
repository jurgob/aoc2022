use std::fs::File;
use std::io::prelude::*;


fn part_1(input: &String) -> i32{

    let lines = input.lines();
    let mut res = 0;
    let mut cur_elf_calories = 0;
    for line in lines {

        if line.trim().is_empty() {
            if cur_elf_calories > res  { res = cur_elf_calories}

            cur_elf_calories = 0;

        } else {
            let calories = line.parse::<i32>().unwrap();
            cur_elf_calories = cur_elf_calories + calories;
        }


    }
    
    res
    

}

fn part_2(input: &String) -> i32{
    let mut vec = Vec::new();

    let lines = input.lines();
    let mut cur_elf_calories = 0;
    for line in lines {

        if line.trim().is_empty() {            
            vec.push(cur_elf_calories);

            cur_elf_calories = 0;

        } else {
            let calories = line.parse::<i32>().unwrap();
            cur_elf_calories = cur_elf_calories + calories;
        }

    }

    vec.sort();

    let len =  vec.len();   
    let res = vec[len - 3] + vec[len - 2] + vec[len - 1];

    res
    

}

// pt 2 
fn main() {
    let mut file = File::open("./input.txt")
        .expect("File not found");

    let mut input = String::new();

    file.read_to_string(&mut input)
        .expect("Error while reading file");

    println!("Part 1 solution: {}", part_1(&input));
    println!("Part 2 solution: {}", part_2(&input));

    

}
