use std::{string, ascii::AsciiExt, ops::Index};

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
    let input = include_str!("../input.txt");
    // let mut lines = input.lines();
    // let line = input.lines().next().unwrap();
    let res_pt1:usize = input.lines().flat_map(| s | {
        both_comp_items(s).chars().collect::<Vec<char>>()
    }).map(|c| {
        item_index(c)
    }).sum();

    println!("res_pt1 {}", res_pt1);
}



