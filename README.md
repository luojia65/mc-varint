# mc-varint
Minecraft VarInt and VarLong implemetation in Rust, providing minimum memory usage and maximum performance.

[![Crates.io][crates-badge]][crates-url]
[![WTFPL licensed][pl-badge]][pl-url]

[crates-badge]: https://img.shields.io/crates/v/mc-varint.svg
[crates-url]: https://crates.io/crates/mc-varint
[pl-badge]: https://img.shields.io/badge/license-WTFPL-blue.svg
[pl-url]: LICENSE

# Example

## Read a VarInt from a Read

```Rust
extern crate mc_varint;
use mc_varint::{VarInt, VarIntRead};
use std::io::Cursor;
fn main() {
    // firstly we create a Cursor
    let mut cur = Cursor::new(vec![0xff, 0xff, 0xff, 0xff, 0x07]);
    // secondly we read from it
    let var_int = cur.read_var_int().unwrap();
    // the value of var_int is 2147483647
    assert_eq!(var_int, VarInt::from(2147483647));
}
```

## Write a VarInt to a Write

```Rust
extern crate mc_varint;
use mc_varint::{VarInt, VarIntWrite};
use std::io::Cursor;
fn main() {
    // firstly we create a Cursor and a VarInt
    let mut cur = Cursor::new(Vec::with_capacity(5));
    let var_int = VarInt::from(2147483647);
    // secondly we write the VarInt to the Cursor
    cur.write_var_int(var_int).unwrap();
    // now the var_int is written to cur.
    assert_eq!(cur.into_inner(), vec![0xff, 0xff, 0xff, 0xff, 0x07]);
}
```

# Performance

Platform: 3.4GHz Intel Core i5

```
running 6 tests
test var_int_convert  ... bench:           7 ns/iter (+/- 0)
test var_int_read     ... bench:          33 ns/iter (+/- 29)
test var_int_write    ... bench:          88 ns/iter (+/- 9)
test var_long_convert ... bench:          10 ns/iter (+/- 1)
test var_long_read    ... bench:          56 ns/iter (+/- 5)
test var_long_write   ... bench:         180 ns/iter (+/- 31)

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured; 0 filtered out
```
