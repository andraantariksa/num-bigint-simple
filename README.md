# Simple Rust Big Integer

I intent to made this **Simple Rust Big Integer** to be used on competitive programming (Especially for solving [Aizu Online Judge - Library of Number Theory](http://judge.u-aizu.ac.jp/onlinejudge/finder.jsp?course=NTL)) and algorithm learning, not for a real world application. If you want to use a more *ready for production* library, you can use [num-bigint](https://github.com/rust-num/num-bigint).

## How to use

Copy [bigint.rs](bigint.rs) to your file and use it like the example below.

## Example use

```rust
// paste simple rust big integer library here

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
        BigInteger::new(num_l[0].clone()),
        BigInteger::new(num_l[1].clone()),
        BigInteger::new(num_l[0].clone()) + BigInteger::new(num_l[1].clone())
    );
    println!(
        "{} - {} = {}",
        BigInteger::new(num_l[0].clone()),
        BigInteger::new(num_l[1].clone()),
        BigInteger::new(num_l[0].clone()) - BigInteger::new(num_l[1].clone())
    );
}
```

## Method

### BigInteger

* new(String) -> BigInteger

### BigUInteger

* new(String) -> BigUInteger

## TODO

- [x] Addition
- [x] Substraction
- [x] Multiplication
- [ ] Multiplication with karatsuba algorithm
- [ ] Division
- [ ] Power
- [ ] Modulo
