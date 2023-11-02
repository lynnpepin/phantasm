# TODOs

Stack:
- Figure out `get_key_ii(token)`. Include state? Or something else?
- Implement tests for above (see recursive resolution)
- Implement it

## Clean up array.rs

Big idea: Simplify things so we use only a few methods.

- Use `get_key_ii(token)` to parse for variable or literal
- If variable, `get_mut_ref` to `Vec<Number>`
  - Keep `ii` around. If `None`, infer `0` in most cases.
- If literal, `parse_numbers` directly from it.
  - 

### Recursive resolution

Outstanding questions:
 - How to parse something like `a.b.c.d` or `a.b.c.1` or `a.b.1.d`?
 - Should I expand `get_token_ii` to include `state`?

Implement recursive `get_key_ii`? E.g. `get_key_ii(a.b.c.d)` should return `a, b.c.d`

E.g.
- `d = [3, 4, 5]`
- `c = [1, 3, 1, 2]`
- `b = [0, 5, 2]`
- `a = [10, 20, 30`]

Examples:
```
get_key_ii("a.b.c.d") -> 
a.b.c.d     d -> d[0]
a.b.c.d.0   d[0] == 3
a.b.c.3     c[3] == 2
a.b.2       b[2] == 2
a.2         
Some("a", Some(2))

get_key_ii("a.b.c.1")
a.b.c.1    c[1] == 3
a.b[3]
How to handle? This is an error

a.b.1.d     1[d]
How to handle? This is an error

a.b.c.2    c[2] == 1
a.b.1      b[1] == 5
Some("a", Some(5)), even though `a[5]` does not exist
```