fn main() {
    let input = include_str!("../input.txt");
    let res_p1= pt_1(input);
    println!("result pt1: {}", res_p1);

}

fn pt_1(input: &str) -> usize{
    let input_chars = input.chars();
    for (i, _c) in input_chars.skip(3).enumerate(){
        let to = i+4;
        let token = &input[i..to];
        if is_key(token) {
            return to;
        }
        
    }
    return 0
}

fn is_key(token: &str) -> bool {
    let v: Vec<char> = token.chars().collect();
    let mut y = v.clone();
    y.sort();
    y.dedup();
    v.len() == y.len()
}


#[cfg(test)]
mod tests {
    use crate::pt_1;
    use crate::is_key;

    #[test]
    fn pt_1_works(){
        assert_eq!(pt_1(&"bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }
    
    
    #[test]
    fn pt_1_works_1(){
        assert_eq!(pt_1(&"nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }
    #[test]
    fn pt_1_works_2(){
        assert_eq!(pt_1(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    }

    #[test]
    fn pt_1_works_3(){
        assert_eq!(pt_1(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
    
    #[test]
    fn is_key_works_true(){
        assert_eq!(is_key(&"vwbj"), true);
    }
    
    #[test]
    fn is_key_works_false(){
        assert_eq!(is_key(&"bvwb"), false);
    }
}