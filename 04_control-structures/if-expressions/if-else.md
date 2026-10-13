# if Expressions

## if.. else

An `if` expression allows you to branch your code depending on conditions. You provide a condition and then state, “If this condition is met, run this block of code. If the condition is not met, do not run this block of code.”

**Syntax**

```rust
fn main() {
    let variable = #;

    if test expression: {
        statement(s);
    } else {
        statement(s);
    }
}
```



## Example 1

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
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
condition was true
```



All `if` expressions start with the keyword `if`, followed by a condition. In this case, the condition checks whether or not the variable `number` has a value less than 5. We place the block of code to execute if the condition is `true` immediately after the condition inside curly brackets. Blocks of code associated with the conditions in `if` expressions are sometimes called *arms*.

Optionally, we can also include an `else` expression, which we chose to do here, to give the program an alternative block of code to execute should the condition evaluate to `false`. If you don’t provide an `else` expression and the condition is `false`, the program will just skip the `if` block and move on to the next bit of code.



## Example 2

```rust
fn main() {
    let number = -3;

    if number >= 0 {
        println!("Positive or Zero");
    } else {
        println!("Negative number");
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
Negative number
```



### Example 3

Write a program to prompt the user for hours and rate per hour using input to compute gross pay. Pay the hourly rate for the hours up to 40 and 1.5 times the hourly rate for all hours worked above 40 hours. Use 45 hours and a rate of 10.50 per hour to test the program (the pay should be 498.75). You should use **input** to read a string and **float()** to convert the string to a number. Do not worry about error checking the user input - assume the user types numbers properly.

```rust
fn main() {
    let hours = 45;
	let rate=1.5;
    
    if hours > 40 {
    	let hours_above= hours-40;
    	let pay=(40*rate) + hours_above*(rate*1.5);
    } else{
        pay=hours*rate;
    }
	println!(pay);
```



**Build / run project**

```bash
cargo build # to build
cargo run # to run
```

**Output**

```bash
Negative number
```



# References

* https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions

