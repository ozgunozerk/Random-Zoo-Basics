use mod_exp::mod_exp;
use primes::is_prime;

/// finds the largest prime that has `l` bits
/// uses prime::is_prime
/// also checks that if p+1 % 4 != 0, so that it will be a "hard" prime

fn find_largest_prime(l: usize) -> usize {
    let n = usize::pow(2, l as u32); // n = 2^l
    for trial in (1..n).step_by(2).rev() {
        if trial % 4 != 3 {
            // ensures p will be a "hard" prime
            if is_prime(trial as u64) {
                return trial;
            }
        }
    }
    return 0;
}

/// computes [Legendre Symbol](https://en.wikipedia.org/wiki/Legendre_symbol).
/// This is necessary for the modular square root computation.
/// Inputs: `p` is a prime, `a` is relatively prime to `p` (if `p` divides `a`, then `a|p = 0`)
/// Returns 1 if a has a square root modulo `p`, -1 otherwise.
/// But since we are working with unsigned integers, it returns 0 instead of -1

fn legendre_symbol(a: usize, p: usize) -> usize {
    let ls = mod_exp(a, (p - 1) / 2, p);
    if ls == (p - 1) {
        return 0;
    } else {
        return ls;
    }
}

/// The Tonelli-Shanks algorithm for finding a quadratic residue (mod p) of `a`.
/// `p` must be an odd prime.
/// Solve the congruence of the form:
/// `x^2 = a (mod p)`
/// And returns x. Note that `p - x` is also a root, thus returning 2 values :)
/// 0 is returned is no square root exists for these `a` and `p`.
/// This algorithm runs in polynomial time
/// (unless the generalized Riemann hypothesis is false).
/// [Reference link](https://eli.thegreenplace.net/2009/03/07/computing-modular-square-roots-in-python)

fn square_root(a: usize, p: usize) -> (usize, usize) {
    if legendre_symbol(a, p) != 1 {
        return (0, 0);
    } else if a == 0 {
        // probably unnecessary for our case
        return (0, 0);
    } else if p == 2 {
        // probably unnecessary for our case
        return (0, 0);
    } else if (p + 1) % 4 == 0 {
        // probably unnecessary for our case
        let res1 = mod_exp(a, (p + 1) / 4, p);
        let res2 = p - res1;
        return (res1, res2);
    } else {
        let mut s = p - 1;
        let mut e = 0;

        while (s % 2) == 0 {
            s /= 2;
            e += 1;
        }
        let mut n = 2;
        while legendre_symbol(n, p) != 0 {
            n += 1;
        }
        let mut x = mod_exp(a, (s + 1) / 2, p);
        let mut b = mod_exp(a, s, p);
        let mut g = mod_exp(n, s, p);
        let mut r = e;

        loop {
            let mut t = b;
            let mut m = 0;
            for trial in 0..r {
                m = trial;
                if t == 1 {
                    break;
                } else {
                    t = mod_exp(t, 2, p);
                }
            }
            if m == 0 {
                return (x, p-x);
            }
            let gs = mod_exp(g, usize::pow(2, r - m - 1), p);
            g = (gs * gs) % p;
            x = (x * gs) % p;
            b = (b * g) % p;
            r = m;
        }
    }
}

/// computes the square of an input `a` in modulo `p`

fn square(a: usize, p: usize) -> usize {
    return mod_exp(a, 2, p);
}

fn main() {
    let modulus = 14;

    let prime = find_largest_prime(modulus);
    match prime {
        0 => println!(
            "Fail! Prime could not be found for any number smaller than {}",
            modulus
        ),
        _ => {
            let plain_text = 12;
            let sqrt = square_root(plain_text, prime);
            let square = square(sqrt.0, prime);

            if square == plain_text {
                println!(
                    "Success! \n\tSquare root of 12 modulo {} is: {}, \n\tsquare of {} modulo {} is: {}",
                    prime, sqrt.0, sqrt.0, prime, square
                );
            } else {
                println!(
                    "Fail! \n\tSquare root of 12 modulo {} is: {}, \n\tsquare of {} modulo {} is: {}",
                    prime, sqrt.0, sqrt.0, prime, square
                );
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn square_test() {
        assert_eq!(9, square(3,13));  // 3**2 = 9 mod 13
        assert_eq!(4, square(2,13));  // 2**2 = 4 mod 13
        assert_eq!(12, square(5,13));  // 5**2 = 12 mod 13
    }

    #[test]
    fn square_root_test() {  // this is a bit longer, since we need to check for two combinations in each scenario
        let mut res = square_root(9, 13);  // 3**2 = 9 mod 13, also 10 is a root
        if res != (3, 10) && res != (10, 3) {
            panic!("Should have been equal to at least one of them");
        }

        res = square_root(4, 13);  // 2**2 = 4 mod 13, also 11 is a root
        if res != (2, 11) && res != (11, 2) {
            panic!("Should have been equal to at least one of them");
        }

        res = square_root(12, 13);  // 5**2 = 12 mod 13, also 8 is a root
        if res != (5, 8) && res != (8, 5) {
            panic!("Should have been equal to at least one of them");
        }
    }

    #[test]
    fn legendre_symbol_test() {
        assert_eq!(1, legendre_symbol(9, 13));  // 3**2 = 9 mod 13
        assert_eq!(1, legendre_symbol(4, 13));  // 2**2 = 4 mod 13
        assert_eq!(1, legendre_symbol(12, 13));  // 5**2 = 12 mod 13
    }

    #[test]
    fn find_largest_prime_test() {
        assert_eq!(241, find_largest_prime(8));  // the largest prime that has 8 bits, is 241
        assert_eq!(113, find_largest_prime(7));  // the largest prime that has 7 bits, is 113
        assert_eq!(61, find_largest_prime(6));  // the largest prime that has 6 bits, is 61
    }

}
