use num_bigint::BigUint;
use num_traits::One;
use num_traits::Zero;

fn main() {
    let n = BigUint::parse_bytes(
        b"19009", 
        10
    ).unwrap();
    
    let generators = find_generators(&n);
    
    println!("Generators of the additive group Z_{}: {:?}", n, generators);
}

/// Finds the generators of the additive group Z_n
fn find_generators(n: &BigUint) -> Vec<BigUint> {
    let mut generators = Vec::new();
    let mut candidate = BigUint::one();
    
    while candidate < *n {
        if is_generator(&candidate, n) {
            generators.push(candidate.clone());
        }
        candidate += BigUint::one();
    }
    
    generators
}

/// Checks if a given number is a generator of the additive group Z_n
fn is_generator(g: &BigUint, n: &BigUint) -> bool {
    let mut current = g.clone();
    let mut count = BigUint::one();

    while current != BigUint::zero() {
        current = (&current + g) % n;
        count += BigUint::one();
        if count == *n {
            break;
        }
    }

    count == *n
}



