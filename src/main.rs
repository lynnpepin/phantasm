use std::io::{stdin,stdout,Write};

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
    loop {
        print!(">>> ");
        let input_string:String = input();
        let input_tokens = input_string.split(" ").collect::<Vec<&str>>();

        print!("\n... {:?}\n", input_tokens);
    }
}

/*
todos
 x loop over input, printing response
 - tokenize each line
 - implement 'set x 0'
 - use arbitrary precision for each value
*/
