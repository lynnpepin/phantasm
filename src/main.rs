use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
mod number;
use number::Number;
mod array;
use array::{parse_numbers, get_value_from_state, get_value, set_value_from_string, set_value};

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

fn main() {
  // Store instructions (list of str), instruction index, state (variable hashmap)
  // Instructions: List of string representing pseudo-ASM instructions
  let mut instructions: Vec<String> = Vec::new();
  // Instruction index: Represents the currently running instruction
  let mut idx: usize = 0;
  // State: Variable hashmap of String key to i64 value
  let mut state: HashMap<String, Vec<Number>> = HashMap::new();
  
  
  loop {
    print!(">>> ");
    let input_string: String = input();
    let input_tokens: Vec<&str> = input_string.split(" ").collect::<Vec<&str>>();
    
    // Operate on each line
    match input_tokens.as_slice() {
      // Directly manipulate and view state
      ["set", kk, vv] => {
        set_value_from_string(&mut state, kk.to_string(), vv.to_string());
      },
      ["del", kk] => {
        state.remove(&kk.to_string());
      },
      ["print"] => { println!("{:?}", state) },
      ["print", kk] => { println!("{:?}", get_value(kk.to_string(), &mut state)); },
      // todo: need to figure out how to just derive all of these from primitive
      //["bprint", kk] => { println!("0b{:b}", get_value(kk, &state)) },
      //["xprint", kk] => { println!("0x{:X}", get_value(kk, &state)) },
      //["oprint", kk] => { println!("0o{:o}", get_value(kk, &state)) }
      
      ["exit"] => {
        println!("exit");
        break;
      },
      // Arithmetic
      [op, kk, kx, ky] => {
        let vx = get_value(kx.to_string(), &mut state).unwrap();
        let vy = get_value(ky.to_string(), &mut state).unwrap();
        set_value(
          &mut state,
          kk.to_string(),
          match op {
            &"add" => vx + vy,
            &"sub" => vx - vy,
            &"mul" => vx * vy,
            &"div" => vx / vy,
            &"and" => vx & vy,
            &"or"  => vx | vy,
            &"xor" => vx ^ vy,
            // We lose the `_` in the outer match, but this is so much nicer
            _ => todo!()
          }
        );
      },
      [op, kk, kx] => {
        let vx = get_value(kx.to_string(), &mut state).unwrap();
        set_value(
          &mut state,
          kk.to_string(),
          match op {
            &"not" => !vx,
            _ => todo!()
          }
        );
      },
      _ => println!("{:?}", input_tokens),
    }
    
    println!("Updated state: {:?}", state);
    idx += 1;
  }
}

/*
TODOs:

- Finish operands
- idx, pc update automatically
- Instruction is indexed by pc
- Store and refer to labels
- Implement `jump if`

- Get instructions from stdin.
- No more interactive input!
- But now we have a list of instructions...
- Tokenize and store in list


Big things:

- Metadata (cycle count), PC in instructions
- Arrays
- Conditions and branches
- Basic programs
- Instructions from stdin. (let someone file > program)
- Instruction index
- Array float index
- Arb precision
- To Wasm
- Webpage IDE

Longterm things:
- Should I be using `&str` or `String`?
- No panics, proper error handling
- Stricter grammar 
- Proper compiler
- Test driven dev
- Clean up the errors, printouts
- Docstrings
- Review all the functions, redundancy, etc.

Nice to have:
- Clean up array.rs
- Clean up printing

DONEs:
- Implement set, state on i64
- Implement add, etc. on i64
- Implement "Number" over i64, f64
- Implement parsing Number from string
- Change get state
- Implement `array`
- Indexed list of numbers stored in `state`
- Implement `input`


*/
