
type Ranges = (usize,usize,usize,usize);

fn string_to_ranges(raw:&str) -> Ranges {
    // let result = ();
    let mut s1 = raw.split(',');
    let mut s2 = s1.next().unwrap().split("-");
    let mut s3 = s1.next().unwrap().split("-");

    let s2a:usize = s2.next().unwrap().parse().unwrap();
    let s2b:usize = s2.next().unwrap().parse().unwrap();

    let s3a:usize = s3.next().unwrap().parse().unwrap();
    let s3b:usize = s3.next().unwrap().parse().unwrap();

    (s2a,s2b,s3a,s3b)
}

fn is_fully_contaned(ranges: Ranges) -> bool {
    
    if ranges.0 <= ranges.2 && ranges.1 >= ranges.3 {
        return true;
    }

    if ranges.0 >= ranges.2 && ranges.1 <= ranges.3 {
        return true;
    }

    false
}

fn is_overlapping(ranges: Ranges) -> bool {
    
    if ranges.0 < ranges.2 && ranges.1 < ranges.2  {
        return false;
    }
    
    if ranges.0 > ranges.3 && ranges.1 > ranges.3 {
        return false;
    }

    true
}

fn pt_1(input: &str) -> usize {
    
    input.lines().filter(|line|{
        let d = string_to_ranges(line);
        is_fully_contaned(d)
    }).count()

}

fn pt_2(input: &str) -> usize {
    
    input.lines().filter(|line|{
        let d = string_to_ranges(line);
        is_overlapping(d)
    }).count()

}

fn main() {
    let input = include_str!("../input.txt");

    println!("pt 1 result: {}", pt_1(input));
    println!("pt 2 result: {}", pt_2(input));
}


#[cfg(test)]
mod tests {
    use crate::string_to_ranges;
    use crate::is_fully_contaned;
    use crate::is_overlapping;
    use crate::pt_1;
    use crate::pt_2;

    #[test]
    fn string_to_ranges_works() {
        let result = string_to_ranges("2-4,6-8");
        assert_eq!(result, (2,4,6,8));
    }
    
    #[test]
    fn is_fully_contaned_works_true_1(){
        assert_eq!(is_fully_contaned((2,8,3,7)), true);
    }

    #[test]
    fn is_fully_contaned_works_true_2(){
        assert_eq!(is_fully_contaned((6,6,4,6)), true);
    }

    #[test]
    fn is_fully_contaned_works_false_1(){
        assert_eq!(is_fully_contaned((1,3,4,6)), false);
    }


    #[test]
    fn is_pt_1_working(){
        let input ="2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(pt_1(input), 2)
    }
    

    #[test]
    fn is_overlapping_working_false(){
        assert_eq!(is_overlapping((2,4,6,8)), false)
    }

    #[test]
    fn is_overlapping_working_false_2(){
        assert_eq!(is_overlapping((2,3,4,5)), false)
    }

    #[test]
    fn is_overlapping_working_false_3(){
        assert_eq!(is_overlapping((4,5,2,3)), false)
    }

    #[test]
    fn is_overlapping_working_true(){
        assert_eq!(is_overlapping((5,7,7,9)), true)
    }

    #[test]
    fn is_overlapping_working_true_2(){
        assert_eq!(is_overlapping((2,8,3,7)), true)
    }

    #[test]
    fn is_overlapping_working_true_3(){
        assert_eq!(is_overlapping((6,6,4,6)), true)
    }

    #[test]
    fn is_overlapping_working_true_4(){
        assert_eq!(is_overlapping((2,6,4,8)), true)
    }

    #[test]
    fn is_pt_2_working(){
        let input ="2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(pt_2(input), 4)
    }

}