use std::{str::FromStr, ops::Div};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", pt_1(input));
}
#[derive(PartialEq)]
#[derive(Debug)]
struct MonkeyRules{
    operation: OperationValue,
    check_test_value: i32,
    if_check_true:usize,
    if_check_false:usize,
}

#[derive(PartialEq, Debug)]
enum OperationValue {
    Add(i32),
    Multiply(i32),
    AddSame,
    MultiplySame,
}

impl FromStr for OperationValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op_string = s.split_once("new = old ").unwrap().1;
        let (operation,operation_value) = op_string.split_once(" ").unwrap();
        match operation {
            "+" => if operation_value == "old" { 
                    Ok(OperationValue::AddSame) 
                } else {
                    Ok(OperationValue::Add(operation_value.parse::<i32>().unwrap()))
                },
            "*" => if operation_value == "old" { 
                Ok(OperationValue::MultiplySame) 
            } else {
                Ok(OperationValue::Multiply(operation_value.parse::<i32>().unwrap()))
            },            
            _ => Err(()),
        }
    }
}


fn parse_operation(input: &str) -> OperationValue {
    // let operation_str= input.split_once("new = old ").unwrap().1;
    // let (operation,operation_value) = lines.get(2).unwrap().split_once("new = old ").unwrap().1.split_once(" ").unwrap();
    input.parse::<OperationValue>().unwrap()
}

fn parse_test_value(input: &str) -> i32 {
    input.split_once(" by ").unwrap().1.parse::<i32>().unwrap()
}

fn parse_throw_to_monkey(input: &str) -> usize {
    input.split_once("throw to monkey ").unwrap().1.parse::<usize>().unwrap()
}

fn apply_operation(val: i32, operation: &OperationValue) -> i32 {
    match operation {
        OperationValue::Add(add_val) => val + add_val,
        OperationValue::Multiply(mul_val) => val * mul_val,
        OperationValue::AddSame => val + val,
        OperationValue::MultiplySame => val * val,
    }
}

fn parse_input(input: &str) -> (Vec<MonkeyRules>,Vec<Vec<i32>>) {
    let mut monkey_rules:Vec<MonkeyRules> = Vec::new();
    let mut monkey_items:Vec<Vec<i32>> =Vec::new();

    input.split("Monkey ")
        .map(|token|token.lines().collect::<Vec<&str>>())
            .filter(|lines|{
                lines.len() > 1    
            }).for_each(| lines| {
            
            let starting_items = lines.get(1).unwrap().split(": ").collect::<Vec<&str>>().get(1).unwrap().split(", ").map(|item|item.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            monkey_items.push(starting_items);

            let operation = parse_operation(lines.get(2).unwrap()); 
            let check_test_value = parse_test_value(lines.get(3).unwrap());
            let if_check_true = parse_throw_to_monkey(lines.get(4).unwrap()) as usize;
            let if_check_false = parse_throw_to_monkey(lines.get(5).unwrap()) as usize;
            monkey_rules.push(MonkeyRules{
                operation: operation,
                check_test_value,
                if_check_true,
                if_check_false,
            });
        }
    );

    (monkey_rules, monkey_items)
}

fn pt_1(input: &str) -> i32 {
    let (monkey_rules,monkey_items_init) = parse_input(input);
    let mut monkey_inspection = monkey_items_init.iter().map(|_mi| 0).collect::<Vec<i32>>(); 
    let mut monkey_items = monkey_items_init.clone();

    for _round in 0..20 {
        for monkey_idx in 0..monkey_rules.len() {
            let cur_monkey_rule = &monkey_rules[monkey_idx];
            let mut cur_monkey_items = monkey_items[monkey_idx].clone();
            
            let cur_monkey_inspection = cur_monkey_items.len() as i32;
            monkey_inspection[monkey_idx] += cur_monkey_inspection;
            
            while cur_monkey_items.len() != 0 {
                let cur_item = cur_monkey_items.pop().unwrap();
                let mut new_item = apply_operation(cur_item, &cur_monkey_rule.operation);
                let new_item_float =  new_item as f32;
                new_item = new_item_float.div(3.0).floor() as i32;
                let test_passed = new_item % cur_monkey_rule.check_test_value == 0;
                if test_passed {
                    monkey_items[cur_monkey_rule.if_check_true].push(new_item);
                } else {
                    monkey_items[cur_monkey_rule.if_check_false].push(new_item);
                }
            }
            monkey_items[monkey_idx] = cur_monkey_items;


        }
    }
    monkey_inspection.sort();
    monkey_inspection.reverse();
    *monkey_inspection.get(0).unwrap_or(&0) *
    *monkey_inspection.get(1).unwrap_or(&0)
    // 0
}


#[cfg(test)]
mod tests_parse_input {
    // use std::ops::Add;
    // use std::ops::Mul;

    use crate::{MonkeyRules, parse_input, parse_operation, parse_test_value,parse_throw_to_monkey,pt_1,OperationValue, apply_operation};

    #[test]
    fn test_parse_operation_add(){
        assert_eq!(parse_operation(&"Operation: new = old + 1"), OperationValue::Add(1));
    }
    
    #[test]
    fn test_parse_operation_multiply_same(){
        assert_eq!(parse_operation(&"Operation: new = old * old"), OperationValue::MultiplySame);
    }
    
    #[test]
    fn apply_operation_add(){
        assert_eq!(apply_operation(3, &OperationValue::Add(2)),5 );
    }
    
    #[test]
    fn apply_operation_multiply_same(){
        assert_eq!(apply_operation(3, &OperationValue::MultiplySame),9 );
    }

    #[test]
    fn test_parse_test_value(){
        assert_eq!(parse_test_value(&"Test: divisible by 23"), 23);
    }

    #[test]
    fn test_parse_throw_to_monkey_true(){
        assert_eq!(parse_throw_to_monkey(&"    If true: throw to monkey 2"), 2);
    }

    #[test]
    fn test_parse_throw_to_monkey_false(){
        assert_eq!(parse_throw_to_monkey(&"    If false: throw to monkey 3"), 3);
    }

    #[test]
    fn test_tests_parse_input_2_monkey() {
        let input = 
"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0";
          
        
        assert_eq!(parse_input(&input), (
            vec![
                MonkeyRules {
                    operation: OperationValue::Multiply(19),
                    check_test_value: 23,
                    if_check_true: 2,
                    if_check_false: 3,
                },
                MonkeyRules {
                    operation: OperationValue::Add(6),
                    check_test_value: 19,
                    if_check_true: 2,
                    if_check_false: 0,
                },
            ],
            vec![
                vec![79, 98],
                vec![54, 65, 75, 74],
            ])
        );
    }
    
    #[test]
    fn test_pt_1() {
        let input = 
"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
          
        
        assert_eq!(pt_1(&input), 101*105) //TODO
    }
}
