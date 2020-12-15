#!/usr/bin/env python

import re

inp = "0,3,6"
inp = "14,8,16,0,1,17"

starting_numbers = inp.split(',')

spoken = {}
spoken_prior = {}

for i in range(1, 30000001):
    if i <= len(starting_numbers):
        last = int(starting_numbers[i - 1])
        spoken[last] = i
        continue

    if not last in spoken_prior.keys():
        last = 0
    else:
        last = spoken[last] - spoken_prior[last]

    if last in spoken.keys():
        spoken_prior[last] = spoken[last]
    spoken[last] = i

    if i == 2020:
        print(i, '>', last)

print(i, '>', last)