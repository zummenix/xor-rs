
extern crate "xor-rs" as xorfn;
extern crate collections;

#[cfg(not(test))]
fn main() {
    let source = [95, 80, 96, 71, 120, 25, 44, 92, 120, 71, 96, 79, 54];
    let result = xorfn::xor(source, [23, 53, 12, 43]);

    match collections::str::from_utf8(result.as_slice()) {
        Some(string) => println!("{}", string),
        _ => {}
    }
}
