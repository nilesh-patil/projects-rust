fn is_prime(n: i32) -> bool {
    // Check if a number is prime

    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn primes_up_to(n: i32) -> Vec<i32> {
    // Return all prime numbers between 1 and n
    let mut primes = Vec::new();
    for i in 1..=n {
        if is_prime(i) {
            primes.push(i);
        }
    }
    primes
}

const N: i32 = 1_000_000;

fn main() {
    // Get all primes between 1 and N
    let primes = primes_up_to(N);
    println!("Primes up to {}: {:?}", N, primes);
}