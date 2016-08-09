/****************
Problem 3:

The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143 ?
****************/

struct Prime {
    curr: u64
}

fn is_prime(num:u64) -> bool {
    for i in 2..(num/2+1) {
        if num % i == 0 {
            return false;
        }
    }

    true
}

impl Iterator for Prime {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let mut temp = self.curr + 1;
        while !is_prime(temp) {
            temp += 1;
        }

        self.curr = temp;
        Some(self.curr)
    }
}

pub fn answer() {
    let mut num = 600851475143u64;
    let mut max_factor = 1;

    for i in (Prime{curr:1}) {
        if num % i == 0 {
            max_factor = i;
        }

        while (num % i ==0) && (num>=2) {
            num /= i;
        }

        if num == 1 {
            break;
        }
    }

    println!("num: {}", max_factor);
}
