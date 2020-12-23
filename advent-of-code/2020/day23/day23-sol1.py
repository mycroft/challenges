#!/usr/bin/env python

import time

contents = "952316487" # my input
rounds = 100
# contents = "389125467" # test
# rounds = 10 # test

cups = list(contents)

for i, v in enumerate(cups):
    cups[i] = int(v)

r = 0

while True:
    current_cup = cups[0]
    pickup = cups[1:4]

    guessed = current_cup - 1
    while True:
        if guessed < min(cups):
            guessed = max(cups)
            continue

        if guessed in pickup:
            guessed -= 1
            continue

        destination = guessed
        break

    remain_cups = cups[4:]

    destination_index = remain_cups.index(destination)

    next_cups = remain_cups[:destination_index+1] + pickup + remain_cups[destination_index+1:] + [cups[0]]

    cups = next_cups

    r+=1
    if r == rounds:
        break

final = cups[cups.index(1)+1:] + cups[:cups.index(1)]

for i, v in enumerate(final):
    final[i] = str(v)

print(''.join(final))

