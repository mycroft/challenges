#!/usr/bin/env python

fd = open('input.txt')
lines = fd.read().split('\n')
fd.close()

preamble_size = 25

current_set = []

for line in lines:
    if line == '':
        print('finished')
        break

    current_value = int(line)

    # print('current_value', current_value)

    if len(current_set) < preamble_size:
        current_set.append(current_value)
        continue

    # print(current_set[len(current_set)-preamble_size:])

    temp_set = current_set[len(current_set)-preamble_size:]
    found = False

    for idx1 in range(preamble_size):
        for idx2 in range(idx1+1, preamble_size):
            if temp_set[idx1] + temp_set[idx2] == current_value:
                found = True
                break
        if found:
            break

    current_set.append(current_value)

    if not found:
        print(current_value)
        break

tofind = 57195069

print("Finding", tofind)

current_total = 0
current_set = []

for line in lines:
    if line == '':
        break

    current_value = int(line)

    print(current_value, current_total, current_set)

    current_set.append(current_value)
    current_total += current_value

    while current_total > tofind:
        removed_value = current_set.pop(0)
        print("removing", removed_value)
        current_total -= removed_value

    if current_total == tofind and len(current_set) > 2:
        print('found a valid set!')
        print(current_set)
        print(min(current_set), max(current_set), min(current_set) + max(current_set))
        break

