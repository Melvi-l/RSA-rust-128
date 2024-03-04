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
    let (pgcd, f_a, _) = euclid(a, modulus);
    println!("a: {}, modulus {}, pgcd {}", a, modulus, pgcd);
    match pgcd == 1 {
        true => Ok(f_a.abs() as u128 % modulus),
        false => Err(Error::F4DoesntFit),
    }
}

#[test]
fn test_mod_mult_inv() {
    let inv = mod_mult_inv(368, 117).unwrap();
    assert_eq!(inv, 55);
}

fn create_key(p: u128, q: u128) -> Result<RSAKey, Error> {
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
fn test_create_key() {
    let key = create_key(104729, 130043).unwrap();
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
    let p = 1_000_000_007;
    let q = 1_000_000_009;
    let n: u128 = p * q;
    assert_eq!(n, 1_000_000_016_000_000_063);
    let key = create_key(p, q).unwrap();
    assert_eq!(key.public.1, F_4);
    assert_eq!(key.private.1, 648946405777194593);
    let message: u128 = 310_400_273_487; // HELLO
    let cipher = encrypt(message, key.public);
    assert_eq!(cipher, 48_060_701_255_754_478);
    let decipher = decrypt(cipher, key.private);
    assert_eq!(decipher, message);
}

fn main() {
    println!("Hello, world !");
}
