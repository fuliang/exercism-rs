pub fn is_prime(n: u32) -> bool {
    if n == 2 {
        return true;
    }

    let m = (n as f64).sqrt() as u32 + 1;

    for i in 2..m {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

pub fn nth(n: u32) -> u32 {
    let mut count = 0;

    for i in 2.. {
        if is_prime(i) {
            count += 1;
        }

        if count == n + 1 {
            return i;
        }
    }
    return 0;
}
