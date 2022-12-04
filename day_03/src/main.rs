
/*

pt 2
For safety, the Elves are divided into groups of three. 
Every Elf carries a badge that identifies their group. 
For efficiency, within each group of three Elves, the badge is the only item type carried by all three Elves.

*/

fn both_comp_items(items: &str) -> String{
    
    let len:usize = (items.len() / 2).try_into().unwrap();
    let first_half = &items[..len];
    let second_half = &items[len..];
    
    let mut res = String::from("");

    for c in first_half.split("") {
        if second_half.contains(c) && !res.contains(c) {
            res.push_str(c);
        }
    }
    res
}

fn item_index(item: char) -> usize {
    let alph = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let idx = alph.find(item).unwrap();
    idx+1
}


fn main() {
    
    //PT 1
    let input = include_str!("../input.txt");
    let res_pt1:usize = input.lines().flat_map(| s | {
        both_comp_items(s).chars().collect::<Vec<char>>()
    }).map(|c| {
        item_index(c)
    }).sum();
    println!("res_pt1 {}", res_pt1);

    //PT 2
    let mut lines_vec:Vec<&str> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        lines_vec.insert(i, line);
    }

    let mut idx = 0;
    let lines_vec_len = lines_vec.len();
    let mut group_items: Vec<char> = Vec::new();
    
    loop {
        let w1 = lines_vec.get(idx).unwrap();
        let w2 = lines_vec.get(idx+1).unwrap();
        let w3 = lines_vec.get(idx+2).unwrap();

        for c in w1.chars() {
            if w2.contains(c) && w3.contains(c){
                group_items.push(c);
                break;
            }
        }

        if idx >= lines_vec_len-3 {
            break;
        }

        idx += 3;    
    }
    let res_pt2:usize = group_items.iter().map(|c| {
        item_index(*c)
    }).sum();
    println!("res_pt2 {:?}", res_pt2);


}



