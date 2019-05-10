extern crate regex;
use std::io;
use regex::Regex;


/**
enum Moves {
    Right = 0,
    Left = 1,
    JumpRight = 2,
    JumpLeft = 3,
}

impl Moves {
    fn from_u8(v: u8) -> String {
        match v {
            0 => String::from("Moves::Right"),
            1 => String::from("Moves::Left"),
            2 => String::from("Moves::JumpRight"),
            3 => String::from("Moves::JumpLeft"),
            _ => panic!(),
        }
    }

} */

fn main() {
    loop {

        println!("Please input the number of frogs for each side.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let n: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!(),
        };

        let puzzle: String = build_fields(&n);

        let mut stack: Vec<String> = Vec::new();

        if dfs_space_solution(&puzzle, &mut stack) {
            println!("Done: {}", stack.len());
            println!("{:?}", stack );
        }
    }
}

fn build_fields(n:&u32) -> String {
    let mut fields = String::from("");

    let mut i = 0;

    while i < *n {
        fields.push('>');

        i = i + 1;
    }

    i = 0;
    fields.push('_');

    while i < *n {
        fields.push('<');

        i = i + 1;
    }

    println!("{}", fields);

    fields
}

fn dfs_space_solution(puzzle: &String, stack: &mut Vec<String>) -> bool {

    stack.push(puzzle.to_string());

    // return if is a solution
    if is_resolved(&puzzle) {
        return true;
    }

    if is_there_wall(&puzzle) {
        stack.pop();
        return false;
    }

    for next_move in 0..=3 {
        let new_state = match try_move(&puzzle, next_move) {
            Some(m) => m,
            None => continue,
        };

        if dfs_space_solution(&new_state, stack) {
            return true;
        }
    }

    stack.pop();
    return false;
}

fn is_resolved(s: &String) -> bool {
    let solution_regex: Regex = Regex::new(r"^<+_>+$").unwrap();
    return solution_regex.is_match(s);
}

fn is_there_wall(s: &String) -> bool {
    let  wall1_regex: Regex = Regex::new(r">><<").unwrap();
    let  wall2_regex: Regex = Regex::new(r"^><<").unwrap();
    let  wall3_regex: Regex = Regex::new(r">><$").unwrap();
    return wall1_regex.is_match(s) || wall2_regex.is_match(s) || wall3_regex.is_match(s);
}

fn try_move(s: &String, my_move: u8) -> Option<String> {
    let index = match s.find('_') {
        Some(i) => i,
        None => panic!(),
    };

    let mut new_s: Vec<char> = s.clone().chars().collect();

    match my_move {
        0 => {
            if index > 0 && new_s[index-1] == '>' {
                new_s[index] = '>';
                new_s[index-1] = '_';
            } else { return None }

        },
        2 => {
            if index > 1 && new_s[index-2] == '>' {
                new_s[index] = '>';
                new_s[index-2] = '_';
            } else { return None }

        },
        1 => {
            if index < new_s.len()-1 && new_s[index+1] == '<' {
                new_s[index] = '<';
                new_s[index+1] = '_';
            } else { return None }
        },
        3 => {
            if index < new_s.len()-2 && new_s[index+2] == '<' {
                new_s[index] = '<';
                new_s[index+2] = '_';
            } else { return None }
        },
        _ => panic!(),
    }

    Some(new_s.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exploration() {
        let mut a = 2;
        assert_eq!(">>_<<", build_fields(&a));
        a = 4;
        assert_eq!(">>>>_<<<<", build_fields(&a));
    }

    #[test]
    fn regex_val() {
        assert!(is_resolved(&String::from("<<<_>>>")));
        assert!(is_resolved(&String::from("<_>")));
    }

    #[test]
    fn regex_wall() {
        assert!(is_there_wall(&String::from(">><<_>>>")));
        assert!(is_there_wall(&String::from("><_>><")));
        assert!(is_there_wall(&String::from("><<_>>")));
    }
}
