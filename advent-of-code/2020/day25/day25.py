#!/usr/bin/env python

subject_number = 7

# Example input:
public_key_card = 5764801
public_key_door = 17807724

# Exercice input:
public_key_card = 10441485
public_key_door = 1004920


def loop(subject_number, value):
    return (subject_number * value) % 20201227

def compute_loop(subject_number = 7, public_key = None, iterations = None):
    value = subject_number
    for i in range(1, 100000000):
        value = loop(subject_number, value)
        if value == public_key or i + 1 == iterations:
            break

    return value, i + 1

card_secret_loop = compute_loop(7, public_key_card)
print(card_secret_loop)

door_secret_loop = compute_loop(7, public_key_door)
print(door_secret_loop)


secret_key_1 = compute_loop(door_secret_loop[0], None, card_secret_loop[1])
secret_key_2 = compute_loop(card_secret_loop[0], None, door_secret_loop[1])

print(secret_key_1, secret_key_2)

if secret_key_1[0] == secret_key_2[0]:
    print(f"Part1: {secret_key_1[0]}")
