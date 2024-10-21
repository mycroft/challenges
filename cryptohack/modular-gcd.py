#!/usr/bin/env python

def euclid_gcd(a, b):
    """
    Compute the greatest common divisor (GCD) of two integers using the Euclidean algorithm.

    Parameters:
    a (int): First integer
    b (int): Second integer

    Returns:
    int: The GCD of a and b
    """
    while b != 0:
        a, b = b, a % b
    return a


print(euclid_gcd(66528, 52920))
