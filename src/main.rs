use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::ops::{Add, Sub, Mul, Div};


// Get input string from stdin
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

// Dynamic-typed number
#[derive(Debug)]
enum Number {
    F64(f64),
    I64(i64),
}

// todo: n identical `impl` for n types. i must learn macros
impl From<i64> for Number {
    fn from(item: i64) -> Self {
        Number::I64(item)
    }
}

impl From<f64> for Number {
    fn from(item: f64) -> Self {
        Number::F64(item)
    }
}

impl Add for Number {
    type Output = Number;
    fn add(self, other: Number) -> Number {
        match (self, other) {
            // todo: 2^n lines for n types, figure out a better way
            (Number::I64(a), Number::I64(b)) => Number::I64(a + b),
            (Number::I64(a), Number::F64(b)) => Number::F64(a as f64 + b),
            (Number::F64(a), Number::I64(b)) => Number::F64(a + b as f64),
            (Number::F64(a), Number::F64(b)) => Number::F64(a + b),
        }
    }
}


impl From<&str> for Number {
    fn from(ss: &str) -> Self {
        if let Ok(vv) = ss.parse::<i64>() {
            Number::I64(vv)
        } else if let Ok(vv) = ss.parse::<f64>() {
            Number::F64(vv)
        } else {
            // todo: do something other than panic
            panic!("yeah i didn't think that would work either");
        }
    }
}


fn _debug_add_main() {
    // yessss
    println!("{:?}", Number::from(1));
    println!("{:?}", Number::from(3.14));
    println!("{:?}", Number::from(1) + Number::from(1));
    println!("{:?}", Number::from(1.0) + Number::from(1));
    println!("{:?}", Number::from(1) + Number::from(1.0));
    println!("{:?}", Number::from(1.0) + Number::from(1.0));
}

fn _debug_from_string() {
    // YESSSSSSSS
    println!("{:?}", Number::from("1"));
    println!("{:?}", Number::from("3.14"));
    println!("{:?}", Number::from("1") + Number::from("1"));
    println!("{:?}", Number::from("1.0") + Number::from("1"));
    println!("{:?}", Number::from("1") + Number::from("1.0"));
    println!("{:?}", Number::from("1.0") + Number::from("1.0"));
}


fn get_value(
    token: &str,
    state: &HashMap<String, i64>
) -> i64 {
    // Parse token as i64 if possible
    // Else, look up in state
    match token.parse::<i64>() {
        Ok(value) => value,
        Err(_) => *state.get(token).unwrap_or(&0)
    }
}



fn notmain() {
    // Store instructions (list of str), instruction index, state (variable hashmap)
    // Instructions: List of string representing pseudo-ASM instructions
    let mut instructions: Vec<String> = Vec::new();
    // Instruction index: Represents the currently running instruction
    let mut idx: usize = 0;
    // State: Variable hashmap of String key to i64 value
    let mut state: HashMap<String, i64> = HashMap::new();


    loop {
        print!(">>> ");
        let input_string: String = input();
        let input_tokens: Vec<&str> = input_string.split(" ").collect::<Vec<&str>>();

        // Operate on each line
        match input_tokens.as_slice() {
            // Directly manipulate and view state
            ["set", kk, vv] => {
                println!("set {} = {}", kk, vv);
                state.insert(kk.to_string(), vv.parse::<i64>().expect("Not a number"));
                println!("Updated state: {:?}", state)
            },
            ["del", kk] => {
                println!("del {}", kk);
                state.remove(&kk.to_string());
                println!("Updated state: {:?}", state)
            },
            ["print"] => { println!("{:?}", state) },
            ["print", kk] => { println!("{}", get_value(kk, &state)); },
            ["bprint", kk] => { println!("0b{:b}", get_value(kk, &state)) },
            ["xprint", kk] => { println!("0x{:X}", get_value(kk, &state)) },
            ["oprint", kk] => { println!("0o{:o}", get_value(kk, &state)) }

            // Arithmetic
            ["add", kk, kx, ky] => {
                println!("add {} {} {}", kk, kx, ky);
                state.insert(
                    kk.to_string(),
                    get_value(kx, &state) + get_value(ky, &state)
                );
                println!("Updated state: {:?}", state);
            },
            ["sub", kk, kx, ky] => {
                println!("sub {} {} {}", kk, kx, ky);
                state.insert(
                    kk.to_string(),
                    get_value(kx, &state) - get_value(ky, &state)
                );
                println!("Updated state: {:?}", state);
            },
            ["mul", kk, kx, ky] => {
                println!("mul {} {} {}", kk, kx, ky);
                state.insert(
                    kk.to_string(),
                    get_value(kx, &state) * get_value(ky, &state)
                );
                println!("Updated state: {:?}", state);
            },
            ["div", kk, kx, ky] => {
                println!("div {} {} {}", kk, kx, ky);
                state.insert(
                    kk.to_string(),
                    get_value(kx, &state) / get_value(ky, &state)
                );
                println!("Updated state: {:?}", state);
            },
            ["exit"] => {
                println!("exit");
                break;
            },
            _ => println!("{:?}", input_tokens),
        }
    }
}
/*

Basic idea:

- Program stores:
    - Instructions list
    - Index of instruction
    - Variable hashmap
        - Values are numbers
        - Arb precision!
        - Labels to jump to are just variables
    - Metadata:
        - Cycle count

TODOs:

- ~~Implement "Number" over i64, f64~~

- ~~Implement parsing Number from string~~

- Change `state` to `Number`, parse `Number` from string...

- Implement all non-jumping instructions. (interactive input)
// Arithmetic: add sub mul div
// Logic:      and, or, xor, not
// (Logic operators are bitwise)
// Conditions: eq, ne, lt, gt, leq, geq
// Bitwise:    shl, shr, rol, ror
// Control flow: label, jump
// Comments! 

- Get instructions from stdin.
    - No more interactive input!
    - But now we have a list of instructions...
    - Tokenize and store in list

- Implement jumping instructions

- Implement some programs...

- Arb precision

*/
