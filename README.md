  # Mini-Aula-Criptografia-RSA
  
  🧠 Mini-Aula: Criptografia RSA – Teoria e Prática
  📘 Introdução à Criptografia
  
  A criptografia é o estudo das técnicas que garantem a confidencialidade, integridade e autenticidade das informações.
  Os algoritmos criptográficos podem ser classificados em dois grandes grupos:
  
  Simétricos: usam a mesma chave para criptografar e descriptografar.
  🔹 Exemplo: AES.
  
  Assimétricos (Chave Pública): usam um par de chaves (pública e privada).
  🔹 Exemplo: RSA.
  
  O RSA (Rivest, Shamir, Adleman) foi desenvolvido em 1977 e é amplamente utilizado em protocolos de segurança como SSL, TLS e SSH.
  
  🔑 Fundamentos Matemáticos do RSA
  
  Números Primos (p, q):
  Base da segurança do RSA. Quanto maiores forem os primos, mais segura é a criptografia.
  
  Função Totiente de Euler (φ):
  
  𝜑
  (
  𝑛
  )
  =
  (
  𝑝
  −
  1
  )
  ×
  (
  𝑞
  −
  1
  )
  φ(n)=(p−1)×(q−1)
  
  É usada para determinar a chave privada.
  
  Teorema de Euler:
  Garante que 
  𝑎
  𝜑
  (
  𝑛
  )
  ≡
  1
  m
  o
  d
   
   
  𝑛
  a
  φ(n)
  ≡1modn, se 
  𝑎
  a e 
  𝑛
  n forem coprimos.
  
  Inverso Modular (d):
  Satisfaz 
  𝑒
  ×
  𝑑
  ≡
  1
  m
  o
  d
   
   
  𝜑
  (
  𝑛
  )
  e×d≡1modφ(n), e é calculado por algoritmos como o Euclidiano Estendido.
  
  ⚙️ Etapas do Algoritmo RSA
  
  Escolha de dois primos grandes: 
  𝑝
  p e 
  𝑞
  q
  
  Cálculo de 
  𝑛
  =
  𝑝
  ×
  𝑞
  n=p×q (módulo público)
  
  Cálculo de 
  𝜑
  (
  𝑛
  )
  φ(n)
  
  Escolha de 
  𝑒
  e (expoente público, ex: 65537)
  
  Cálculo de 
  𝑑
  d (inverso modular de 
  𝑒
  e)
  
  Formação das chaves:
  
  🔓 Chave Pública: 
  (
  𝑒
  ,
  𝑛
  )
  (e,n)
  
  🔒 Chave Privada: 
  (
  𝑑
  ,
  𝑛
  )
  (d,n)
  
  🔐 Criptografia e Descriptografia
  
  Criptografia:
  
  𝐶
  =
  𝑀
  𝑒
  m
  o
  d
   
   
  𝑛
  C=M
  e
  modn
  
  (usa a chave pública)
  
  Descriptografia:
  
  𝑀
  =
  𝐶
  𝑑
  m
  o
  d
   
   
  𝑛
  M=C
  d
  modn
  
  (usa a chave privada)
  
  🧩 Segurança e Desafios
  
  A segurança do RSA baseia-se na dificuldade de fatorar números grandes (problema da fatoração).
  
  Requer alto custo computacional, mas oferece elevada segurança.
  
  Padrões modernos recomendam chaves de 2048 bits ou mais.
  
  🚫 Limitações
  
  Lento em comparação com algoritmos simétricos (como AES).
  
  Vulnerável a ataques Side-Channel, Padding Oracle e Timing se mal implementado.
  
  ⚠️ Ameaça Quântica
  
  Computadores quânticos poderão quebrar RSA usando o algoritmo de Shor, o que motiva pesquisas em criptografia pós-quântica.

🛠️ Passo a passo: Criando e usando o código RSA em Rust
1️⃣ Instalar Rust

Se ainda não tiver Rust, instale pelo site oficial:

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2️⃣ Criar um novo projeto Rust

cargo new memoria_demo
cd memoria_demo

3️⃣ Adicionar dependências no Cargo.toml

    [package]
    name = "memoria_demo"
    version = "0.1.0"
    edition = "2024"
    
    [dependencies]
    num-bigint = "0.4"
    num-traits = "0.2"

4️⃣ Criar o código RSA
Substitua o conteúdo de src/main.rs por:

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

5️⃣ Executar o código
                
    cargo run

Você verá a saída mostrando:

Chave pública e privada

Mensagem original

Mensagem criptografada

Mensagem descriptografada

