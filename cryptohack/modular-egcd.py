#!/usr/bin/env python

def extended_gcd(a, b):
    """
    Compute the greatest common divisor (GCD) of two integers using the Extended Euclidean algorithm.

    Parameters:
    a (int): First integer
    b (int): Second integer

    Returns:
    tuple: A tuple containing the GCD and the coefficients x and y such that a*x + b*y = extended_gcd(a, b)
    """
    if a == 0:
        return b, 0, 1
    else:
        gcd, x1, y1 = extended_gcd(b % a, a)
        x = y1 - (b // a) * x1
        y = x1
        return gcd, x, y


res = extended_gcd(26513, 32321)
print(res)
