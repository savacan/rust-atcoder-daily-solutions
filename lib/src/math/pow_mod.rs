pub fn pow_mod(a: usize, pow: usize, m: usize) -> usize {
    if pow == 0 {
        1
    } else if pow == 1 {
        a
    } else if pow % 2 == 0 {
        (pow_mod(a, pow / 2, m) * pow_mod(a, pow / 2, m)) % m
    } else {
        (pow_mod(a, pow - 1, m) * a) % m
    }
}
