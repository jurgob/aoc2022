
/* 
The score for a single round is the score for the shape you selected
    Rock (A,X) => 1 
    Paper (B,Y) => 2
    Scissors (C,Z) => 3

Outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)

pt2: 
X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win

 */
fn match_score(s: &str) -> Result<i32, ()> {
    match s {
        //p1 win
        "AZ" => Ok(0),
        "BX" => Ok(0),
        "CY" => Ok(0),
        // drow
        "AX" => Ok(3),
        "BY" => Ok(3),
        "CZ" => Ok(3),
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


fn translate(s: &str) -> Result<&str,()> {
    match s {
        // p1 win (you loose)
        "AX" => Ok("AZ"),
        "BX" => Ok("BX"),
        "CX" => Ok("CY"),
        //draw
        "AY" => Ok("AX"),
        "BY" => Ok("BY"),
        "CY" => Ok("CZ"),
        //p2 win (you wint)
        "AZ" => Ok("AY"),
        "BZ" => Ok("BZ"),
        "CZ" => Ok("CX"),
        _ => Err(()),
    }
}

fn raw_to_score(result: &str) -> i32{
    
    let my_figure = result.chars().last().unwrap();
    let score = match_score(&result).unwrap() + figure_score(&my_figure).unwrap() ;
    score
}

fn main() {
    let input = include_str!("../input.txt");

    let result:i32 = input.lines().map(|res| {
        let result = res.replace(" ", "");
        raw_to_score(&result)
    }).sum(); 

    let result_pt2:i32 = input.lines().map(|res| {
        let res_not_translate = res.replace(" ", "");
        let result =translate(&res_not_translate).unwrap(); 
        raw_to_score(&result)
    }).sum(); 
  
    println!("part 1 result: {:?}", result);
    println!("part 2 result: {:?}", result_pt2);
}
