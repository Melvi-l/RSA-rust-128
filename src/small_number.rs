use std::u128;

const F_4: u128 = 65_537;

// Key generation

#[derive(Debug)]
enum Error {
    F4DoesntFit,
}

struct RSAKey {
    public: (u128, u128),
    private: (u128, u128),
}

fn euclid(a: u128, b: u128) -> (u128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let r = a % b;
        let q = (a / b) as i128;
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
    assert_eq!(f_a * 240 + f_b * 46, pgcd as i128)
}

fn mod_mult_inv(a: u128, modulus: u128) -> Result<u128, Error> {
    let (pgcd, mut f_a, _) = euclid(a, modulus);
    println!("a: {}, modulus {}, pgcd {}", a, modulus, pgcd);
    match pgcd == 1 {
        true => {
            while f_a < 0 {
                f_a += modulus as i128;
            }
            Ok(f_a as u128 % modulus)
        }
        false => Err(Error::F4DoesntFit),
    }
}

#[test]
fn test_mod_mult_inv() {
    let inv = mod_mult_inv(368, 117).unwrap();
    assert_eq!(inv, 62);
    assert_eq!(
        mod_mult_inv(111059998755241, 115788865422351189).unwrap(),
        45094773746044996
    );
}

fn generate_key(p: u128, q: u128) -> Result<RSAKey, Error> {
    let n = p * q;
    let phi_n = (p - 1) * (q - 1);
    let e = F_4;
    let d = match mod_mult_inv(e, phi_n) {
        Ok(inv) => inv,
        Err(err) => return Err(err),
    };
    Ok(RSAKey {
        public: (n, e),
        private: (n, d),
    })
}

#[test]
fn test_generate_key() {
    let key = generate_key(104729, 130043).unwrap();
    assert_eq!(key.public, (13619273347, F_4));
    assert_eq!(key.private, (13619273347, 4992975569));
}

// Encryption/Decryption

fn quick_expo(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
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
        base = (base * base) % modulus
    }
    result
}

#[test]
fn test_quick_expo() {
    assert_eq!(quick_expo(2, 5, 13), 6);
    assert_eq!(quick_expo(10, 2, 1000), 100);
    assert_eq!(quick_expo(2, 10, 1024), 0);
    assert_eq!(quick_expo(7, 0, 13), 1);
    assert_eq!(quick_expo(0, 10, 13), 0);
    assert_eq!(quick_expo(3, 3, 1), 0);
}

fn encrypt(message: u128, public_key: (u128, u128)) -> u128 {
    quick_expo(message, public_key.1, public_key.0)
}

#[test]
fn test_encrypt() {
    let cipher = encrypt(1234, (1022117, 65537));
    assert_eq!(cipher, 412611);
}

fn decrypt(cipher: u128, private_key: (u128, u128)) -> u128 {
    quick_expo(cipher, private_key.1, private_key.0)
}

#[test]
fn test_decrypt() {
    let message = decrypt(412611, (1022117, 832193));
    assert_eq!(message, 1234);
}

#[test]
fn integration() {
    let p = 1000000007;
    let q = 1000000009;
    let n: u128 = p * q;
    let phi_n: u128 = (p - 1) * (q - 1);
    assert_eq!(n, 1000000016000000063);
    let key = generate_key(p, q).unwrap();
    assert_eq!(key.public.1, 65537);
    assert_eq!(key.private.1, 648946405777194593);
    assert_eq!((key.public.1 * key.private.1) % phi_n, 1);
    let message: u128 = 310400273487; // HELLO
    let cipher = encrypt(message, key.public);
    assert_eq!(cipher, 48060701255754478);
    let decipher = decrypt(cipher, key.private);
    assert_eq!(decipher, message);
}

fn pow(mut base: i128, mut exp: i128) -> i128 {
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result *= base;
        }
        exp >>= 1;
        base *= base
    }
    result
}

#[test]
fn test_pow() {
    assert_eq!(pow(2, 3), 8);
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(240, 46), 2);
}

fn compute_jacobi_symbol(mut a: i128, mut b: i128) -> i128 {
    if a == 1 || b == 1 {
        return 1;
    }
    if a == 0 {
        return 0;
    }
    if a % 2 == 0 {
        return compute_jacobi_symbol(2, b) * compute_jacobi_symbol(a/2, b)
    }
    if a >= 0 {
        let exp = (a - 1) * (b - 1) / 4;
        return compute_jacobi_symbol(b % a, a) * pow(-1, exp as i128)
    }
    panic!("shit ain't workin");
}

#[test]
fn test_jacobi() {
    assert_eq!(compute_jacobi_symbol(12345, 6789), -1);
}
// fn generate_prime_number() -> (u128, u128) {
//
// jacobi}
