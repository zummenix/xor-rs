xor-rs
======

A repeating-key XOR function written in Rust.<br />
This function might be useful to play with [the matasano crypto challenges](http://cryptopals.com).

###Example

This example prints `Hello, world!`

In your `Cargo.toml` 
```toml
[dependencies.xor]
git = "https://github.com/zummenix/xor-rs/"

```

In your `main.rs`
```rust
extern crate xor;
extern crate collections;

fn main() {
    let source = [95, 80, 96, 71, 120, 25, 44, 92, 120, 71, 96, 79, 54];
    let result = xor::xor(source, [23, 53, 12, 43]);

    match collections::str::from_utf8(result.as_slice()) {
        Some(string) => println!("{}", string),
        _ => {}
    }
}
```

LICENSE
-------

MIT
