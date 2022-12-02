
/* 
The score for a single round is the score for the shape you selected
    Rock (A,X) => 1 
    Paper (B,Y) => 2
    Scissors (C,Z) => 3

Outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)
 */
fn match_score(s: &str) -> Result<i32, ()> {
    match s {
        // drow
        "AX" => Ok(3),
        "BY" => Ok(3),
        "CZ" => Ok(3),
        //p1 win
        "AZ" => Ok(0),
        "BX" => Ok(0),
        "CY" => Ok(0),
        //p2 win
        "CX" => Ok(6),
        "AY" => Ok(6),
        "BZ" => Ok(6),
        _ => Err(()),
    }
}

fn figure_score(s: &char) -> Result<i32, ()> {
    match s {
        'X' => Ok(1),
        'Y' => Ok(2),
        'Z' => Ok(3),
        _ => Err(()),
    }
} 

fn main() {
    let input = include_str!("../input.txt");

    let result:i32 = input.lines().map(|res| {
        let result = res.replace(" ", "");
        let my_figure = result.chars().last().unwrap();
        let score = match_score(&result).unwrap() + figure_score(&my_figure).unwrap() ;
        score
    }).sum(); 
  
    println!("result: {:?}", result);
}
