const F_4: u64 = 65_537;

#[derive(Debug)]
enum Error {
    F4DoesntFit,
}
fn quick_expo(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        result = (result * result) % modulus
    }
    result
}

struct RSAKey {
    public_key: (u64, u64),
    private_key: (u64, u64),
}

fn pgcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        pgcd(b, a % b)
    }
}

#[test]
fn test_pgcd() {
    let pgcd = pgcd(255, 141);
    assert_eq!(pgcd, 3);
}

fn euclid(a: u64, b: u64) -> (u64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let r = a % b;
        let q = (a / b) as i64;
        let (x, f_a, f_b) = euclid(b, r);
        (x, f_b, f_a - q * f_b)
    }
}

#[test]
fn test_euclid() {
    let (pgcd, f_a, f_b) = euclid(240, 46);
    assert_eq!(pgcd, 2);
    assert_eq!(f_a, -9);
    assert_eq!(f_b, 47);
    assert_eq!(f_a * 240 + f_b * 46, pgcd as i64)
}

fn mod_mult_inv(a: u64, modulus: u64) -> Result<u64, Error> {
    let (pgcd, f_a, _) = euclid(a, modulus);
    println!("a: {}, modulus {}, pgcd {}", a, modulus, pgcd);
    match pgcd == 1 {
        true => Ok(f_a.abs() as u64 % modulus),
        false => Err(Error::F4DoesntFit),
    }
}

#[test]
fn test_mod_mult_inv() {
    let inv = mod_mult_inv(368, 117).unwrap();
    assert_eq!(inv, 55);
}

fn create_key(p: u64, q: u64) -> Result<RSAKey, Error> {
    let n = p * q;
    let phi_n = (p - 1) * (q - 1);
    let e = F_4;
    let d = match mod_mult_inv(e, phi_n) {
        Ok(inv) => inv,
        Err(err) => return Err(err),
    };
    Ok(RSAKey {
        public_key: (n, e),
        private_key: (n, d),
    })
}

#[test]
fn test_create_key() {
    let key = create_key(104729, 130043).unwrap();
    assert_eq!(key.public_key, (13619273347, F_4));
    assert_eq!(key.private_key, (13619273347, 4992975569));
}

fn main() {
    println!("Hello, world !");
}
