/***************************
Problem 1:

if we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
#Find the sum of all the multiples of 3 or 5 below 1000.
***************************/

pub fn answer() {
    let mut result = 0;

    for x in 0..1000 {
        match (x%3, x%5) {
            (0, _) => result += x,
            (_, 0) => result += x,
            _ => continue
        }
    }

    println!("result: {}", result)
}
