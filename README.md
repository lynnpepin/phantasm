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

Operators:

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

TODO:

- **

---

This language is named after "Penumbra Phantasm", a long-sought after Homestuck song by Toby Fox. It was never released in full, but its instrumentation and melody is referenced in many tracks. It's kind of a "white whale" for Homestuck music fans. Most importantly, the word ends in "asm". You can read about Penumbra Phantasm [on the Homestuck Music Wiki](https://hsmusic.wiki/track/penumbra-phantasm/), listen to [the archived 2012 livestream version](https://www.youtube.com/watch?v=RIq4GrMv96I), or the partial [Homestuck Soundtest version](https://www.youtube.com/watch?v=OdntMzdkFnk).
