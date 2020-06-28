# Simple Rust Big Integer

A simple Rust Big Integer library in one file. Easy to use and to copy-paste.

I intent to made this **Simple Rust Big Integer** to be used on competitive programming (Especially for solving [Aizu Online Judge - Library of Number Theory](http://judge.u-aizu.ac.jp/onlinejudge/finder.jsp?course=NTL)) and algorithm learning.

## How to use

Copy [bigint.rs](bigint.rs) to your file and use it like the example below.

## Example use

Copy `num-bigint-simple` from `src/lib.rs` and paste on the code where you want to use biginteger.

```rust
// Paste simple rust big integer library in the code

use bigint::BigInteger;

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).ok();
    let num_l: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();
    println!(
        "{} + {} = {}",
        BigInteger::from_str(&num_l[0]),
        BigInteger::from_str(&num_l[1]),
        BigInteger::from_str(&num_l[0]) + BigInteger::from_str(&num_l[1])
    );
    println!(
        "{} - {} = {}",
        BigInteger::from_str(&num_l[0]),
        BigInteger::from_str(&num_l[1]),
        BigInteger::from_str(&num_l[0]) - BigInteger::from_str(&num_l[1])
    );
}
```

## Features Implemented

- [x] Addition
- [x] Substraction
- [x] Multiplication
- [ ] Multiplication with karatsuba algorithm
- [ ] Division
- [ ] Power
- [ ] Modulo

## License

Licensed under [MIT License](LICENSE)
