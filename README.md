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
