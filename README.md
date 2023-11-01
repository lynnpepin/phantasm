# Phantasm - A fantasy Assembly

Phantasm is a fantasy assembly language with:

 - Named variables, no registers
 - Interpreted. *For no reason! Your code runs on strings and you are going to like it.*
 - Dynamic typing between `f64` and `i64`. *Who needs unsigned ints?*
 - No formal grammar, no tree sitter, no language server.

```
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

Phantasm is super unstable because I am still writing it.

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

# You can define arrays too, but operands are not vectorized between them
set some_array 10,20,30,40,50
# some_value = some_array[2] = 30
get some_value some_array 2

# Jumps: TODO
```

# Operators:

- **Arithmetic** operators cast to float when necessary:
  - `add k x y`
  - `sub k x y`
  - `mul k x y`
  - `div k x y` always casts to float.
    - Use `div k k 1` to convert `k` from int to float.
- **Bitwise operators** always cast to int.
  - `and k x y`
  - ` or k x y`
  - `xor k x y`
  - `not k x`
- Later TODO:
  - **Branching and jumps:** `label`, `jump`, `_pc`, `_cycles`
  - **Other bitwise ops:**  `shl, shr, rol, ror`
  - **Math ops:** TODO, `round`, `floor,` `ceil`, `float`
  - **Cooler arithmetic operators:** `pow`, `log`, `float`, and constants like `e`, `pi`, even `i`. Imagine.
  - **Array ops:** `psh` (to end), `pop` (to end), `deq` (from start), `len`, `cat` (two arrays, or `cat val arr` to start)

---

This language is named after "Penumbra Phantasm", a long-sought after Homestuck song by Toby Fox. It was never released in full, but its instrumentation and melody is referenced in many tracks. It's kind of a "white whale" for Homestuck music fans. Most importantly, the word ends in "asm". You can read about Penumbra Phantasm [on the Homestuck Music Wiki](https://hsmusic.wiki/track/penumbra-phantasm/), listen to [the archived 2012 livestream version](https://www.youtube.com/watch?v=RIq4GrMv96I), or the partial [Homestuck Soundtest version](https://www.youtube.com/watch?v=OdntMzdkFnk).

---

Some tests:

```
set x 0,1.0,2.0,3.0
set y 0,10,20,30,40
add z x.3 y.4
# expect z = [F64(43.0)]


set x 0,1.0,2.0,3.0
set y 0,10,20,30,40
add z.4 x.3 y.4
# expect z = [0, 0, 0, 43.0]

# Loop from i = 0 to i = 3
set i 0
set N 3
loop:
add i i 1
sub left N i
jif left loop
```