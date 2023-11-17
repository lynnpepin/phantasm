use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
mod number;
use number::Number;
mod array;
use array::{parse_numbers, get_value_from_state, get_value, set_value_from_string, set_value, set_value_in_vec_in_state};


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
  // State: Variable hashmap of String key to i64 value
  let mut state: HashMap<String, Vec<Number>> = HashMap::new();

  // Initialize state
  state.insert("__label___start".to_string(), vec![Number::I64(0)]);
  state.insert("__pc".to_string(), vec![Number::I64(0)]);
  state.insert("__cc".to_string(), vec![Number::I64(0)]);
  state.insert("__verbose".to_string(), vec![Number::I64(1)]);
  
  
  loop {
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
      print!(">>> ");
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
      ["set", kk, ii, vv] => {
        set_value_in_vec_in_state(
          &mut state,
          kk.to_string(),
          ii.parse::<usize>().ok(),
          *parse_numbers(&vv.to_string()).unwrap().get(0).unwrap(),
        );
      },
      ["get", kk, kx, ii] => {
        // kk = kx[ii]
        let idx = get_value(ii.to_string(), &mut state).unwrap().to_i64() as usize;
        let vv = get_value_from_state(
          &mut state,
          kx.to_string(),
          Some(idx)
        ).unwrap();
        set_value(
          &mut state,
          kk.to_string(),
          vv,
        );
      },
      ["del", kk] => {
        state.remove(&kk.to_string());
      },
      ["input", kk] => {
        set_value_from_string(&mut state, kk.to_string(), input());
      },
      ["input", kk, ii] => {
        set_value_in_vec_in_state(
          &mut state,
          kk.to_string(),
          ii.parse::<usize>().ok(),
          *parse_numbers(&input()).unwrap().get(0).unwrap(),
        );
      },
      ["print"] => { println!("{:?}", state) },
      ["print", kk] => { println!("{:?}", get_value(kk.to_string(), &mut state)); },
      
      [label] if label.ends_with(":") => {
        set_value(
          &mut state,
          format!("__label_{}", label.to_string().replace(":","")),
          Number::I64(_pc as i64)
        );
      },

      ["jif", kk, label] => {
        let vv = get_value(kk.to_string(), &mut state).unwrap();
        let label_idx = get_value_from_state(
          &mut state,
          format!("__label_{}", label.to_string()),
          Some(0)
        ).unwrap();
         
        if vv >= Number::F64(1.0) {
          set_value(
            &mut state,
            "__pc".to_string(),
            label_idx + Number::I64(1),
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
            &"eq"  => Number::I64((vx == vy) as i64),
            &"neq" => Number::I64((vx != vy) as i64),
            &"gt"  => Number::I64((vx > vy) as i64),
            &("geq" | "gte") => Number::I64((vx >= vy) as i64),
            &"lt"  => Number::I64((vx < vy) as i64),
            &("leq" | "lte") => Number::I64((vx <= vy) as i64),
            // We lose the `_` in the outer match, but this is so much nicer
            _ => todo!()
          }
        );
      },
      // Other ops
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
      // do nothing
      _ => (),
      //println!("{:?}", input_tokens),
    }
    let verbose = get_value("__verbose".to_string(), &mut state).unwrap();
    if verbose >= Number::I64(1) {
      if verbose >= Number::I64(2) {
        if verbose >= Number::I64(3) {
          println!("{:?}", instructions);
        } else {
          println!("{};", instruction_string);
        }
      }
      println!("Updated state: {:?}", state);
    }

 
  }
}

/*
TODOs:
- Implement set/get for arrays
- Implement A.x.y.z indexing
- Implement branch versions of eq/ords
- Implement bubblesort and array get/set matches


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

DONEs:
- Implement `input()` op
- Get instructions from stdin.
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
- Implement lt, gt, leq, geq, eq, neq
- Implement constants, and vars like `time()`
- Printing on/off by setting `__verbose`
- Redo `label` as namespaced `_label_{label}`
- Clean up printing


*/
