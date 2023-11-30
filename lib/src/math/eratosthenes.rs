// エラトステネスの篩
// n: usize までの素数を列挙する
pub fn seive_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut primes = vec![];
    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
            let mut j = 2;
            while j * i <= n {
                is_prime[j * i] = false;
                j += i;
            }
        }
    }
    primes
}
