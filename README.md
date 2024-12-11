This repository contains the assembler library for the group project.

# Overview
Library provides a trait `Assembler` with two associated functions: `disassemble` and `reassemble` to fragmentize and reassemble data according to the AP protocol. It also provides a simple struct `NaiveAssembler` implementing the trait.

# Usage
```rust
// Assume json is a String-type value
let bytes: &[u8] = json.as_bytes();
// Create fragments
let fragments: Vec<Fragment> = NaiveAssembler::disassemble(bytes);
// Reassemble fragments into byte vector
let bytes: Vec<u8> = NaiveAssembler::reassemble(&fragments);

```
