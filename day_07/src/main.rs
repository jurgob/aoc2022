use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    
    println!("pt 1: {}", pt_1(&input));
    println!("pt 2: {}", pt_2(&input));
    
}

struct Terminal {
    pwd: String
}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            pwd: String::from("/")
        }
    }

    fn cd(&mut self, path: &str) {
        if path == ".." {
            let mut path_split = self.pwd.split("/").collect::<Vec<&str>>();
            path_split.pop();
            self.pwd = path_split.join("/");
        } else if path == "/" {
            self.pwd = String::from("/");
        }else {
            if self.pwd == "/" {
                self.pwd = format!("{}{}", self.pwd, path);
            } else {
                self.pwd = format!("{}/{}", self.pwd, path);
            }
        }
    }

    fn get_pwd(&self) -> String {
        self.pwd.clone()
    }
}


fn pt_1(input: &str) -> usize { 
    let mut t = Terminal::new();
    let mut dirs_info = HashMap::new();

    input.split("$ ").for_each(|cmd_token|{
        // let (cmd, output) = cmd_token.split_once("\n").unwrap();
        let cmd = cmd_token.split_once("\n");

        match cmd {
            Some((cmd, output)) => {

                if cmd.starts_with("cd ") {
                    let path_move = cmd.split(" ").nth(1).unwrap();
                    t.cd(path_move);
                } else if cmd.starts_with("ls") {
                    let lines = output.lines();
                    let dir_size: usize = lines.filter(|line| {
                        !line.starts_with("dir ")
                    }).map(|line|{
                        let size : usize = line.split_once(" ").unwrap().0.parse().unwrap();
                        size
                    }).sum();
                    
                    let pwd = t.get_pwd(); 
                    // println!("dir insert info: {} size: {}, exists: {}", pwd, dir_size, dirs_info.contains_key(&t.get_pwd()));
                    // println!("- pwd: {}", pwd);
                    dirs_info.insert(pwd, dir_size);
                    
                    let pwd1 = t.get_pwd();

                    // increment parents size
                    if pwd1 != "/" {
                        let mut pwd_split:Vec<&str> = pwd1.split("/").collect();
                        pwd_split.pop();
                        while pwd_split.len() > 0 {
                            let mut p_dir:String = pwd_split.join("/");
                            if p_dir == "" {
                                p_dir = "/".to_string();
                            }
                            
                            // println!(" - p_dir: {}", p_dir);
                            let prev_size = dirs_info.get(&p_dir).unwrap();
                            let p_dir_size = prev_size + dir_size;
                            dirs_info.insert(p_dir, p_dir_size);
                            pwd_split.pop();
                            
                        }
                    }
                    

                }
            },
            None => {
            }
        }
    });
    
    // println!("dirs_info: {:#?}", dirs_info);

    let dirs_size_total: usize = dirs_info.values().filter(|val | val <= &&100000 ).sum();
    dirs_size_total
}

fn pt_2(input: &str) -> i32 { 
    let mut t = Terminal::new();
    let mut dirs_info = HashMap::new();

    input.split("$ ").for_each(|cmd_token|{
        // let (cmd, output) = cmd_token.split_once("\n").unwrap();
        let cmd = cmd_token.split_once("\n");

        match cmd {
            Some((cmd, output)) => {

                if cmd.starts_with("cd ") {
                    let path_move = cmd.split(" ").nth(1).unwrap();
                    t.cd(path_move);
                } else if cmd.starts_with("ls") {
                    let lines = output.lines();
                    let dir_size: i32 = lines.filter(|line| {
                        !line.starts_with("dir ")
                    }).map(|line|{
                        let size : i32 = line.split_once(" ").unwrap().0.parse().unwrap();
                        size
                    }).sum();
                    
                    let pwd = t.get_pwd(); 
                    // println!("dir insert info: {} size: {}, exists: {}", pwd, dir_size, dirs_info.contains_key(&t.get_pwd()));
                    // println!("- pwd: {}", pwd);
                    dirs_info.insert(pwd, dir_size);
                    
                    let pwd1 = t.get_pwd();

                    // increment parents size
                    if pwd1 != "/" {
                        let mut pwd_split:Vec<&str> = pwd1.split("/").collect();
                        pwd_split.pop();
                        while pwd_split.len() > 0 {
                            let mut p_dir:String = pwd_split.join("/");
                            if p_dir == "" {
                                p_dir = "/".to_string();
                            }
                            
                            // println!(" - p_dir: {}", p_dir);
                            let prev_size = dirs_info.get(&p_dir).unwrap();
                            let p_dir_size = prev_size + dir_size;
                            dirs_info.insert(p_dir, p_dir_size);
                            pwd_split.pop();
                            
                        }
                    }
                    

                }
            },
            None => {
            }
        }
    });
    
    // println!("dirs_info: {:#?}", dirs_info);
    let disk_space_total = 70000000;
    let disk_space_needed = 30000000;
    let dirs_size_total: i32 = dirs_info.values().sum();
    let mut res = std::i32::MAX;

    for (_dir, size) in dirs_info {
        let new_disk_space = (disk_space_total - dirs_size_total) + size;
        println!("_dir {} size {} new_disk_space {}", _dir, size, new_disk_space);
        if new_disk_space > disk_space_needed {
            if size < res {
                res = size;
            }
        }
    }

    res
    
}
#[cfg(test)]
mod tests_terminal {
    use crate::Terminal;

    #[test]
    fn terminal_works_no_cd(){
        let t = Terminal::new();
        assert_eq!(t.get_pwd(), "/");
    }

    #[test]
    fn terminal_works_1_sub_dir(){
        let mut t = Terminal::new();
        t.cd("my_dir");
        assert_eq!(t.get_pwd(), "/my_dir");
    }
    
    #[test]
    fn terminal_works_root(){
        let mut t = Terminal::new();
        t.cd("my_dir");
        t.cd("/");
        assert_eq!(t.get_pwd(), "/");
    }

    #[test]
    fn terminal_works_2_sub_dir(){
        let mut t = Terminal::new();
        t.cd("my_dir");
        t.cd("my_dir_sub");
        assert_eq!(t.get_pwd(), "/my_dir/my_dir_sub");
    }


    #[test]
    fn terminal_works_sub_dir_1_back(){
        let mut t = Terminal::new();
        t.cd("my_dir");
        t.cd("my_dir_sub");
        t.cd("..");
        assert_eq!(t.get_pwd(), "/my_dir");
    }

    #[test]
    fn terminal_works_2_sub_dir_root(){
        let mut t = Terminal::new();
        t.cd("my_dir");
        t.cd("my_dir_sub");
        t.cd("/");
        assert_eq!(t.get_pwd(), "/");
    }

}

#[cfg(test)]
mod tests_p1 {
    use crate::{pt_1, pt_2};
    
//     #[test]
//     fn p1_works_only_root() {
//         let input = "$ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// ";
//         let res = vec![14848514, 8504156].iter().filter(|val| val >= &&100000).sum();
//         assert_eq!(pt_1(&input), res);
//     }

//     #[test]
//     fn p1_works_root_and_1_sub_dir() {
//         let input = "$ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst";
//     let res = vec![14848514, 8504156,29116,2557,62596].iter().filter(|val| val >= &&100000).sum();

//         assert_eq!(pt_1(&input), res );
//     }

    #[test]
    fn p1_works() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(pt_1(&input), 95437);
    }

    #[test]
    fn p2_works() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
8033020 d.ext
7214296 k";

        assert_eq!(pt_2(&input), 24933642);
    }
}

// 40913445