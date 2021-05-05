use hardprime;

#[test]
fn test_add() {
    let num = 13;
    let prime = 23;
    let sqrt = hardprime::square_root(num, prime);
    let square = hardprime::square(sqrt.0, prime);
    if num != square {
        panic!("We took the square root of {} modulo {}, and got {}, then take the square of {} modulo {}, and got {}", 
                num, prime, sqrt.0, sqrt.0, prime, square);
    }
}