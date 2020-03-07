fn is_prime(n: u32) -> bool {
    if n == 2 {
        return true;
    }

    let root: u32 = (n as f64).sqrt().ceil() as u32;

    for possible_factor in 2..=root {
        if n % possible_factor == 0 {
            return false;
        }
    }

    true
}

pub fn sieve(n: u32) -> Vec<u32> {
    let all: Vec<u32> = (2..n).collect();
    let mut acc: Vec<u32> = Vec::new();

    for i in all {
        if is_prime(i) {
            acc.push(i);
        }
    }

    acc
}
