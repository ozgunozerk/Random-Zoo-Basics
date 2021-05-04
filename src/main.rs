#[doc(inline)]
use mod_exp::mod_exp;
use primes::is_prime;

/// finds the largest prime, which is smaller than the given input `n`
/// uses prime::is_prime

fn find_largest_prime(n: usize) -> usize {
    for trial in (0..n).rev() {
        if is_prime(trial as u64) {
            return trial;
        }
    }
    return 0;
}

/// computes [Legendre Symbol](https://en.wikipedia.org/wiki/Legendre_symbol).
/// This is necessary for the modular square root computation.
/// Inputs: `p` is a prime, `a` is relatively prime to `p` (if `p` divides `a`, then `a|p = 0`)
/// Returns 1 if a has a square root modulo `p`, 0 otherwise.

fn legendre_symbol(a: usize, p: usize) -> usize {
    let ls = mod_exp(a, (p - 1) / 2, p);
    if ls == (p - 1) {
        return 0;
    } else {
        return ls;
    }
}

/// Find a quadratic residue (mod p) of `a`. 
/// `p` must be an odd prime.
/// Solve the congruence of the form:
/// `x^2 = a (mod p)`
/// And returns x. Note that `p - x` is also a root.
/// 0 is returned is no square root exists for these `a` and `p`.
/// The Tonelli-Shanks algorithm is used (except
/// for some simple cases in which the solution
/// is known from an identity). This algorithm runs in polynomial time 
/// (unless the generalized Riemann hypothesis is false).
/// [Reference link](https://eli.thegreenplace.net/2009/03/07/computing-modular-square-roots-in-python) 

fn square_root(a: usize, p: usize) -> usize {
    if legendre_symbol(a, p) != 1 {
        return 0;
    } else if a == 0 {
        return 0;
    } else if p == 2 {
        return 0;
    } else if (p + 1) % 4 == 0 {
        return mod_exp(a, (p + 1) / 4, p);
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
                return x;
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
            let square = square(sqrt, prime);

            if square == plain_text {
                println!(
                    "Success! \n\tSquare root of 12 modulo {} is: {}, \n\tsquare of {} modulo {} is: {}",
                    prime, sqrt, sqrt, prime, square
                );
            } else {
                println!(
                    "Fail! \n\tSquare root of 12 modulo {} is: {}, \n\tsquare of {} modulo {} is: {}",
                    prime, sqrt, sqrt, prime, square
                );
            }
        }
    }
}
