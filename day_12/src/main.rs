use std::{collections::HashMap, ops::Index};


fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", pt_1(input));
}


type HeightMap = Vec<Vec<i32>>;

fn pt_1(input: &str) -> i32  {
    let map = input_to_map(input);
    let root = find_val(&map, 0);
    let dest = find_val(&map, 27);
    let arches = map_2_arches(root, &map);
    // println!("arches: {:#?}", arches);
    shortest_path(root, dest,&arches)
}

fn shortest_path(root: Node, dest: Node, arches: &HashMap<Node, Vec<Node>>) -> i32 {
    println!("shortest_path");
    let mut queue:Vec<Node> = Vec::new();
    let mut visited: HashMap<Node, bool> = HashMap::new();
    let mut distance_to_node = HashMap::new();
    // queue.push((0,0,0));
    distance_to_node.insert(root, (0, root));
    
    queue.push(root);
    while !queue.is_empty() {
        let node = queue.pop().unwrap();
        println!("node: {:?}", node);
        let children    = arches.get(&node).unwrap();
        let cur_dist = distance_to_node.get(&node).unwrap().clone().0;
        for child in children {
            
            let new_dist = cur_dist + 1;
            if !distance_to_node.contains_key(child) ||  distance_to_node.get(child).unwrap().0 > new_dist {
                distance_to_node.insert(*child, (new_dist, node));
            }

            if !visited.contains_key(child) {
                queue.push(*child);
            }
        }
        visited.insert(node, true);
        // let node_val = node.2;
        // if node_val == 26 {
        //     println!("distance_to_node: ");
        //     distance_to_node.iter().for_each(|(k,v)| {
        //         println!("node: {:?}: dist{:?}, parent:{}", k, v.1, v.0);
        //     });
        //     return distance_to_node.get(&node).unwrap().clone().0;
        // }
    }   
    println!("distance_to_node: ");
    
    distance_to_node.iter().for_each(|(node,(dist, parent))| {
        let parent_c = "SabcdefghijklmnopqrstuvwxyzE".chars().collect::<Vec<char>>().index(parent.2 as usize).clone();
        let node_c = "SabcdefghijklmnopqrstuvwxyzE".chars().collect::<Vec<char>>().index(node.2 as usize).clone();
        println!("{:02}  - node: {:?}: parent {:?}, dist: {}. {} <- {}", dist, node, parent, dist, node_c, parent_c);
    });

    
    distance_to_node
        .iter()
        .filter(|(k,_v)| if k.2 == 27 { true } else { false })
        .next().unwrap().1.0
    // return distance_to_node.get(&dest).unwrap().clone().0;
    
}

fn input_to_map(input: &str) -> HeightMap {
    
    let alphabet = "SabcdefghijklmnopqrstuvwxyzE".chars().collect::<Vec<char>>();
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            let h:i32 = alphabet.iter().position(|&x| x == c).unwrap().try_into().unwrap();
            row.push(h);
        }
        map.push(row);
    }
    
    // let map_print = map.iter().map(|row| row.iter().map(|x|{
    //     let prefix = if x < &10 { "  " } else { " " };
    //     prefix.to_owned() + x.to_string().as_str() + " "
    // } ).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("\n");
    // println!("map_print: \n{}", map_print);

    map
}

type Node = (usize, usize, i32);
fn find_val (map: &HeightMap, val: i32) -> Node {
    let mut root:(usize,usize, i32) = (0, 0, 0);
    for h in 0..map.len() {
        for w in 0..map[0].len() {
            if map[h][w] == val {
                root = (h, w, 0);
            }
        }
    }
    root
}


fn map_2_arches(root: Node, map: &HeightMap) -> HashMap<Node, Vec<Node>> {
    let mut arches:HashMap<Node, Vec<Node>> = HashMap::new();
    let mut queue:Vec<Node> = Vec::new();
    queue.push(root);
    while !queue.is_empty() {
        let map_w = map[0].len();
        let map_h = map.len();
        let node = queue.pop().unwrap();
        let h = node.0;
        let w = node.1;
        let val = node.2;
        // println!("node: {:?}", node);
        // println!("queue: {:?}", queue);
        let mut adiacent = Vec::new();
        if w > 0 { 
            adiacent.push((h, w-1, map[h][w-1]));
        }
        if w < map_w - 1 { 
            adiacent.push((h, w+1, map[h][w+1]));
        }
        if h > 0 { 
            adiacent.push((h-1,w, map[h-1][w] ));
        };
        if h < map_h - 1 { 
            adiacent.push(( h+1, w, map[h+1][w] ));
        };
        let mut children = Vec::new();

        adiacent.iter().for_each(|c_node| {
            if c_node.2 <= val+1 {
                children.push(*c_node);
                if !arches.contains_key(c_node) { 
                    queue.push(*c_node);
                }
            }
        });
        
        // println!("children: {:?}", children);

        arches.insert(node, children);
        // println!("arches: {:?}", arches);

        
    }
    arches
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pt_1() {
        let input = 
"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(31, pt_1(input));
    }
}
