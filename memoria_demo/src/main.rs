use num_bigint::{BigUint, ToBigUint};
    use num_traits::{One, Zero};
    
    fn modular_inverse(a: &BigUint, m: &BigUint) -> BigUint {
        let (mut t, mut new_t) = (BigUint::zero(), BigUint::one());
        let (mut r, mut new_r) = (m.clone(), a.clone());
    
        while new_r != BigUint::zero() {
            let q = &r / &new_r;
    
            let temp_t = new_t.clone();
            new_t = if &t > &(&q * &new_t) {
                (&t - &q * &new_t) % m
            } else {
                (m - ((&q * &new_t) - &t) % m) % m
            };
            t = temp_t;
    
            let temp_r = new_r.clone();
            new_r = &r - &q * &new_r;
            r = temp_r;
        }
    
        t % m
    }
    
    fn modular_pow(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
        base.modpow(exponent, modulus)
    }
    
    fn generate_rsa_keys() -> ((BigUint, BigUint), (BigUint, BigUint)) {
        let p = 71u32.to_biguint().unwrap();
        let q = 67u32.to_biguint().unwrap();
    
        let n = &p * &q;
        let phi = (&p - 1u32) * (&q - 1u32);
        let e = 19u32.to_biguint().unwrap();
        let d = modular_inverse(&e, &phi);
    
        ((n.clone(), e), (n, d))
    }
    
    fn rsa_encrypt(message: &BigUint, public_key: &(BigUint, BigUint)) -> BigUint {
        let (n, e) = public_key;
        modular_pow(message, e, n)
    }
    
    fn rsa_decrypt(ciphertext: &BigUint, private_key: &(BigUint, BigUint)) -> BigUint {
        let (n, d) = private_key;
        modular_pow(ciphertext, d, n)
    }
    
    fn main() {
        let (public_key, private_key) = generate_rsa_keys();
    
        println!("==================== RSA DEMO ====================");
        println!(">>> Chave Pública  : (n = {}, e = {})", public_key.0, public_key.1);
        println!(">>> Chave Privada  : (n = {}, d = {})", private_key.0, private_key.1);
        println!("=================================================\n");
    
        let mensagem = 123u32.to_biguint().unwrap();
        println!("Mensagem Original: {}", mensagem);
    
        let cifrado = rsa_encrypt(&mensagem, &public_key);
        println!("Mensagem Criptografada: {}", cifrado);
    
        let decifrado = rsa_decrypt(&cifrado, &private_key);
        println!("Mensagem Decifrada   : {}", decifrado);
    
        println!("\n✅ Operação concluída com sucesso!");
    }