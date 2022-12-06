fn main() {
    let input = include_str!("../input.txt");
    println!("result pt1: {}", pt_1(input));
    println!("result pt2: {}", pt_2(input));

}

fn pt_1(input: &str) -> usize{
    find_token(input,4)
}

fn is_key(token: &str) -> bool {
    let v: Vec<char> = token.chars().collect();
    let mut y = v.clone();
    y.sort();
    y.dedup();
    v.len() == y.len()
}

fn find_token(input: &str, token_size:usize) -> usize{
    let input_chars = input.chars();
    for (i, _c) in input_chars.skip(token_size-1).enumerate(){
        let to = i+token_size;
        let token = &input[i..to];
        if is_key(token) {
            return to;
        }
        
    }
    return 0
}

fn pt_2(input: &str) -> usize{
    find_token(input,14)
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

#[cfg(test)]
mod tests_pt1 {
    use crate::pt_1;

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
}

#[cfg(test)]
mod tests_pt_2 {
    use crate::pt_2;

    #[test]
    fn pt_2_works(){
        assert_eq!(pt_2(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    } 
    
    #[test]
    fn pt_2_works_1(){
        assert_eq!(pt_2(&"bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }
    #[test]
    fn pt_2_works_2(){
        assert_eq!(pt_2(&"nppdvjthqldpwncqszvftbrmjlhg"), 23);
    }

    #[test]
    fn pt_2_works_3(){
        assert_eq!(pt_2(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }

    #[test]
    fn pt_2_works_4(){
        assert_eq!(pt_2(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
