# if Expressions

## if

An `if` expression allows you to branch your code depending on conditions. You provide a condition and then state, “If this condition is met, run this block of code. If the condition is not met, do not run this block of code.”

**Syntax**

```rust
fn main() {
    let variable = #;

    if test expression: {
        statement(s);
}
```



### Example 1

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```



**Build / run project**

```bash
cargo build # to build
cargo run # to run
```

**Output**

```bash
number was something other than zero
```



### Example 2



```rust
fn main() {
    let number = 0;

    if number > 0 {
        println!("number is a positive number");
    }
}
```



**Build / run project**

```bash
cargo build # to build
cargo run # to run
```

**Output**

```bash
number is a positive number
```



### Example 3

Let’s try changing the value of `number` to a value 0 that makes the condition `false` to see what happens:

```rust
fn main() {
    let number = 0;

    if number > 0 {
        println!("number is a positive number");
    }
}
```



**Build / run project**

```bash
cargo build # to build
cargo run # to run
```

**Output**

```bash

```

* No output of this example



### Example 4

It’s also worth noting that the condition in this code *must* be a `bool`. If the condition isn’t a `bool`, we’ll get an error. For example, try running the following code:

```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```



**Build / run project**

```bash
cargo build # to build
cargo run # to run
```

**Output**

The `if` condition evaluates to a value of `3` this time, and Rust throws an error:

```bash
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
```



The error indicates that Rust expected a `bool` but got an integer. Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide `if` with a Boolean as its condition. 

