#!/usr/bin/env python

fd = open('input.txt', 'r')
contents = fd.read().split('\n')
fd.close()

data = []

for line in contents:
    if line == '':
        break

    data.append(line)

slopes = [
    [1, 1],
    [3, 1],
    [5, 1],
    [7, 1],
    [1, 2]
]

total = 1

for slope in slopes:
    idx_j = 0
    idx_i = 0

    trees = 0
    while idx_j < len(data):
        # print(idx_i, idx_j, data[idx_j][idx_i%31])
        if data[idx_j][idx_i%31] == '#':
            trees += 1

        idx_i += slope[0]
        idx_j += slope[1]

    total *= trees
    #print(trees)

print(total)