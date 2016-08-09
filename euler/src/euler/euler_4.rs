/*************
Problem 4:
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
Find the largest palindrome made from the product of two 3-digit numbers.
*************/

fn is_palindrome(num: u32) -> bool {
    let s = num.to_string();

    for (h, t) in s.chars().zip(s.chars().rev()) {
        if h != t {
            return false;
        }
    }

    true
}

pub fn answer() {
    let mut max = 0u32;

    for i in 100..999 {
        for j in 100..999 {
            let temp = i * j;
            if is_palindrome(temp) && temp > max {
                max = temp;
            }
        }
    }

    println!("max: {}", max)
}
