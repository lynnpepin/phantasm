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

  // Labels: Used for jumping
  let mut labels: HashMap<String, usize> = HashMap::new();

  // Initialize state
  labels.insert("__start".to_string(), 0);
  state.insert("__pc".to_string(), vec![Number::I64(0)]);
  state.insert("__cc".to_string(), vec![Number::I64(0)]);
  
  
  loop {
    print!(">>> ");
    // Increment PC and Cycle counter
    let _pc = get_value_from_state(
      &mut state,
      "__pc".to_string(),
      Some(0)
    ).unwrap().to_i64() as usize;

    let _cc = get_value_from_state(
      &mut state,
      "__cc".to_string(),
      Some(0)
    ).unwrap();

    set_value(
      &mut state,
      "__pc".to_string(),
      Number::I64(_pc as i64) + Number::I64(1)
    );

    set_value(
      &mut state,
      "__cc".to_string(),
      _cc + Number::I64(1),
    );
    
    // Get new instruction and tokenize it
    if instructions.len() <= _pc {
      instructions.push(input());
    }
    let instruction_string: String = instructions.get(_pc).unwrap().to_string();
    let input_tokens: Vec<&str> = instruction_string
      .split_whitespace()
      .collect::<Vec<&str>>()
    ;
    
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
      [label] if label.ends_with(":") => {
        labels.insert(
          label
            .to_string()
            .replace(":", ""),
          _pc
        );
        println!("Updated labels: {:?}", labels);
      },
      ["jif", kk, label] => {
        let vv = get_value(kk.to_string(), &mut state).unwrap();
        let label_idx = labels.get(*label).unwrap().to_owned();
        if vv != Number::I64(0) {
          set_value(
            &mut state,
            "__pc".to_string(),
            Number::I64((label_idx + 1) as i64)
          )
        } else {
          Ok(())
        };
      },
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
 
  }
}

/*
TODOs:
- Implement lt, gt, leq, geq, eq, neq
- Implement branch versions of above
- Implement `input()`
- Get instructions from stdin.


Big things:

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
- idx, pc update
- Add to instructions each input
- Instruction indexed by pc
- Store and refer to labels
- Implement `jump if`
- ~~No more interactive input!
- But now we have a list of instructions...
- Tokenize and store in list
- Metadata (cycle count), PC in instructions
- Arrays


*/
