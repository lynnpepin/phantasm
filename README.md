# Phantasm - A fantasy Assembly

Phantasm is a fantasy assembly language with these phantastic pheatures:

 - Named variables, no registers
 - Interpreted. *Your code runs on strings and you are going to like it.*
 - Dynamic typing between `f64` and `i64`. *Who needs unsigned ints and static types?*
 - No formal grammar, no tree sitter, no language server.
 - Slow: Estimated 500KHz clock speed on an M1 Pro CPU

```asm
# Find max
set __verbose 0
set A 3,1,20,6,2,16,20,19,3,17
set i 0
set N 10
set x -1

set_max_as_x:
add max x 0

dont_set_max_as_x:
get x A i
gt new_is_larger x max
jif new_is_larger set_max_as_x

add i i 1
sub left N i
jif left dont_set_max_as_x

print max
# Ok(20)
```

```asm
>>> set s 1000 
set s = 1000
Updated state: {"s": I64(1000)}
>>> set r 1.02 
set r = 1.02
Updated state: {"s": I64(1000), "r": F64(1.02)}
>>> mul s s r
mul s s r
Updated state: {"s": F64(1020.0), "r": F64(1.02)}
>>> mul s s r
mul s s r
Updated state: {"s": F64(1040.4), "r": F64(1.02)}
>>> mul s s r
mul s s r
Updated state: {"s": F64(1061.208), "r": F64(1.02)}
>>> yessssssssssss
["yessssssssssss"]
```

Phantasm is super unstable because I am still writing it! Don't use this!

---


I wrote this language because it's something I wanted when I was learning to write MIPS in an undergrad course.  My main debugging pains came from trying to differentiate structural structural problems from lower-level problems. Was the structure of my code incorrect? Or was it a finer implementation gotcha?

The tool I wanted was a higher-level, pseudo-assembly language. So I produced a little toy, Phantasm.

Turns out Phantasm is the name of a famous horror movie... And hey, this is an interpreted, dynamically typed, memory-unsafe language. That's pretty scary!

---

# Basic usage

```
# Instantiate variables by name.
# Variables can't be numbers, but other than that, go wild
set x 1
set y 2.3

# Addition: z = x + y
# Z is instantiated, and is dynamically cast to to float
add z x y

comments are implicit btw. anything not interpreted as an op is a comment, including this line 

# You can define arrays too, zero-indexed
# operands are not vectorized between them. you can't add two arrays
set some_array 10,20,30,40,50

# you can index arrays for convenience
add blah.4 100 some_array.2
# blah = [0, 0, 0, 0, 130]
# you can't write "blah.x" to get blah[1] though

# some_value = some_array[2] = 30
get some_value some_array 2

# Jumps: TODO
```

# Operators:
- **Get, set, view variables:**
  - `set z v` *assigns `z = v`*
  - `set z i v` *assigns `z[i] = v`
    - TODO: Make it possible for `i` to be another variable rather than int literal
  - `get z k i` *assigns `z = k[i]`*
  - `del k`
  - `print k`
  - `input z` *assigns z from user input`
  - `input z i` *assigns z[i] from user input`
  - `len z A` *assigns Z = len(A)`. *TODO*
- **Arithmetic** operators cast to float when necessary:
  - `add k x y`
  - `sub k x y`
  - `mul k x y`
  - `div k x y` always casts to float.
    - Use `div k k 1` to convert `k` from int to float.
- **Bitwise / bit operators** always cast to int.
  - `and k x y`
  - ` or k x y`
  - `xor k x y`
  - `not k x`
  - TODO: shl, shr, rol, ror
- **Control flow:** `k` is truthy if `k[0] >= 1.0`
  - `label:`
  - `jif k label` *jump to label if v != 0*
  - TODO: jeq, jneq, jgt, jgte/jgeq, jlt, jlte/jleq
- Later TODO:
  - **Other bitwise ops:**  `shl, shr, rol, ror`
  - **Math ops:** TODO, `round`, `floor,` `ceil`, `float`
  - **Cooler arithmetic operators:** `pow`, `log`, `float`, and constants like `e`, `pi`, even `i`. Imagine.
  - **Array ops:** `psh` (to end), `pop` (to end), `deq` (from start), `len`, `cat` (two arrays, or `cat val arr` to start)

---

# Under the hood

- The entire state is stored under a list of `instructions: Vec<String>` and a `state: HashMap<String, Vec<Number>>`.
- **Dynamic typing:** `Number` is a custom type that defines ops between `i64` and `f64` for dynamic typing.
- **Instructions:** Instructions are interpreted at each "cycle" and cannot be directly modified by running code.
- **Variables:** Variables are stored in `state`. There are not scalars, just one-long `Vec<Number>`.
- **Special variables** store program state. These are the program counter (`__pc`), the cycle counter (`__cc`), and labels (`__label_{label}`).
- **Program counter:** Stored in `state` as `__pc`, and can be modified (e.g. as `set __pc 123`)
- **Labels:** These are also stored in `state`, as `__label_{label_name}`, and can be directly modified (e.g. as `set __label_loop 123`.)
  - `__start` is a special label, defined as `__label___start = 0`.


# Why is it called Phantasm?

This language is named after "Penumbra Phantasm", a long-sought after Homestuck song by Toby Fox. It was never released in full, but its instrumentation and melody is referenced in many tracks. It's kind of a "white whale" for Homestuck music fans. Most importantly, the word ends in "asm". You can read about Penumbra Phantasm [on the Homestuck Music Wiki](https://hsmusic.wiki/track/penumbra-phantasm/), listen to [the archived 2012 livestream version](https://www.youtube.com/watch?v=RIq4GrMv96I), or the partial [Homestuck Soundtest version](https://www.youtube.com/watch?v=OdntMzdkFnk).

# Architecture:

- `main.rs` loops over provided instructions.
- ~~`state.rs`: Programs are stored in `State`, which is a `HashMap<String, Vec<Number>>`.~~
- `array.rs`: Every variable is an array of `Number`, and those operations are defined here.
- `number.rs` defines the base `Number` type which provides dynamic typing between `f64` and `i64`.
- `array.rs` 

---

Some tests / examples

```
# Array indexing
set x 0,1.0,2.0,3.0
set y 0,10,20,30,40
add z x.3 y.4
# expect z = [F64(43.0)]


set x 0,1.0,2.0,3.0
set y 0,10,20,30,40
add z.4 x.3 y.4
# expect z = [0, 0, 0, 43.0]

# Loop from i = 0 to i = 3
set i 0.0
set N 3
loop:
add i i 1
sub left N i
jif left loop

# Loop indefinitely
set i 0
loop:
add i i 1
jif 1 loop

# Loop and count
# Took ~2 seconds on M1 Pro when not printing
set i 0.0
set N 1000000
print 1
loop:
add i i 1
sub left N i
jif left loop



# Find max -- written without label preprocessing
set __verbose 0
set A 3,1,20,6,2,16,20,19,3,17
set i 0
set N 10
set x -1

set_max_as_x:
add max x 0

dont_set_max_as_x:
get x A i
gt new_is_larger x max
jif new_is_larger set_max_as_x

add i i 1
sub left N i
jif left dont_set_max_as_x

print max

```