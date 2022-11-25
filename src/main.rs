use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
//use rug::{Integer, Ratioal, Float, Assign};

fn input() -> String {
    let mut ss = String::new();
    // Ensure chars get printed before the prompt
    stdout().flush(); 
    stdin().read_line(&mut ss).expect("Broken string");

    // Remove newlines if the terminal brings them in
    if let Some('\n') = ss.chars().next_back() {
        ss.pop();
    }
    if let Some('\r') = ss.chars().next_back() {
        ss.pop();
    }

    ss
}


fn main() {
    let mut state: HashMap<String, String> = HashMap::new();

    loop {
        print!(">>> ");
        let input_string: String = input();
        let input_tokens: Vec<&str> = input_string.split(" ").collect::<Vec<&str>>();

        // Operate on each line
        match input_tokens.as_slice() {
            ["set", kk, vv] => {
                println!("set {} = {}", kk, vv);
                state.insert(kk.to_string(), vv.to_string());
                println!("Updated state: {:?}", state);
            },
            ["del", kk] => {
                println!("del {}", kk);
                state.remove(&kk.to_string());
                println!("Updated state: {:?}", state);
            },
            ["add", kk, kx, ky] => {
                println!("add {} {} {}", kk, kx, ky);
                let x: i64 = kx.trim().parse().expect("Not a number");
                let y: i64 = ky.trim().parse().expect("Not a number");
                let z: i64 = x + y;
                state.insert(kk.to_string(), z.to_string());
                println!("Updated state: {:?}", state);
            },
*/
            ["exit"] => {
                println!("exit");
                break;
            },
            _ => println!("{:?}", input_tokens),
        }
    }
}

/*
todos
 x loop over input, printing response
 x tokenize each line
 - each line is saved by integer
 x implement 'set x 0'
 - implement 'get x', 'del x',
 - hashmap can take String, i64, or f64
 - implement add, sub
 - use arbitrary precision for each value, casting from i64 -> Integer -> Rational -> Float as needed
*/
