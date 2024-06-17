# Simple TOTP Generator

Generate your TOTP auth code.

This generator uses sha256 as crypto function.

## Installation

Add `simple_totp_generator = "0.1.0"` under your `[dependencies]` in `Cargo.toml`

## Usage

```rs
use simple_totp_generator::{generate_totp};

fn main() {
    //                       ↓↓↓ Shared secret  ↓↓↓ Time step
    let totp = generate_totp("THISISASECRETKEY", 60);
    println!("Your TOTP is: {}", totp);
}
```
