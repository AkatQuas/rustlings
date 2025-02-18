// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!
//
// check out the page
// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#the--operator-can-be-used-in-functions-that-return-result

use std::error::Error;
use std::num::ParseIntError;
///
/// `Box<dyn Error>` to mean **“any kind of error”**. Using `?` in a main function with this return type is allowed.
///
fn main() -> Result<(), Box<dyn Error>> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
