#!/usr/bin/env python

from Crypto.Util.number import getStrongPrime, bytes_to_long, long_to_bytes

def mod_inverse(e, phi):
    def extended_gcd(a, b):
        if a == 0:
            return b, 0, 1
        else:
            gcd, x, y = extended_gcd(b % a, a)
            return gcd, y - (b // a) * x, x

    gcd, x, _ = extended_gcd(e, phi)
    if gcd != 1:
        raise ValueError("Modular inverse does not exist")
    else:
        return x % phi

def compute_private_exponent(p, q, e):
    n = p * q
    phi = (p - 1) * (q - 1)
    d = mod_inverse(e, phi)
    return d

p = getStrongPrime(2048)
q = getStrongPrime(2048)
e = 2 ** 16 + 1

secret_content = "Am I Wrong - Etienne de Crecy"

print(f"{secret_content = }")

# n is our private key
n = p * q

print()
print("public key:")
print(f"{n = }")
print(f"{e = }")

cypher = pow(bytes_to_long(secret_content.encode("utf-8")), e, n)

print(f"{cypher = }")

# decrypting the cipher:
# - computing the private exponent (with p, q and e)
# - using it to decrypt our content

print()
print("secret key:")
print(f"{p = }")
print(f"{q = }")

priv_e = compute_private_exponent(p, q, e)

print("f{priv_e = }")

m = pow(cypher, priv_e, n)

plain_text = long_to_bytes(m).decode('utf-8')

print()
print(f"{plain_text = }")
