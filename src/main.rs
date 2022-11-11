use std::io::{stdin,stdout,Write};
use std::collections::HashMap;


fn input() -> String {
    let mut ss = String::new();
    stdout().flush();  // Ensure chars get printed before the prompt

    stdin().read_line(&mut ss).expect("Broken string");

    // Remove newlines if the terminal brings them in
    if let Some('\n')=ss.chars().next_back() { ss.pop(); }
    if let Some('\r')=ss.chars().next_back() { ss.pop(); }

    ss
}

fn main() {
    let mut state: HashMap<&str, &str> = HashMap::new();
    let mut input_string: String;
    let mut input_tokens: Vec<&str>;

    loop {
        print!(">>> ");
        //let input_string: String = input();
        //let input_tokens: Vec<&str> = input_string.split(" ").collect::<Vec<&str>>();
        input_string = input();
        input_tokens = input_string.split(" ").collect::<Vec<&str>>();
        // operate on each input
        match input_tokens.as_slice() {
            ["set", kk, vv] => {
                println!("{}{}", kk, vv); // THIS works... But not updating state.
                state.insert(kk.clone(), vv.clone());
                //println!("Updated state: {:?}", state);
            },
            _ => println!("{:?}", input_tokens)
        }
    }
}

/*
todos
 x loop over input, printing response
 x tokenize each line
 - implement 'set x 0'
 - use arbitrary precision for each value
*/
