#!/usr/bin/env python

fd = open('input.txt')
lines = fd.read().splitlines()
fd.close()

flipped = []

for line in lines:
    idx = 0
    current_i = 0.0
    current_j = 0.0

    while idx < len(line):
        if line[idx] == 'e':
            current_i += 1

        elif line[idx] == 'w':
            current_i -= 1

        elif line[idx] == 'n':
            current_j -= 1
            if line[idx+1] == 'e':
                current_i += 0.5
            elif line[idx+1] == 'w':
                current_i -= 0.5

            idx += 1

        elif line[idx] == 's':
            current_j += 1
            if line[idx+1] == 'e':
                current_i += 0.5
            elif line[idx+1] == 'w':
                current_i -= 0.5

            idx += 1

        idx += 1

    t = (current_i, current_j)
    if t not in flipped:
        flipped.append(t)
    else:
        flipped.remove(t)

print(f"Part 1: {len(flipped)}")

def deltas():
    return [
        (1, 0),
        (-1, 0),
        (0.5, 1),
        (-0.5, 1),
        (-0.5, -1),
        (0.5, -1),
    ]

def get_adj(tile):
    adj_tiles = []
    for delta in deltas():
        adj_tiles.append(((tile[0] + delta[0]), (tile[1] + delta[1])))
    return adj_tiles

def get_black_adj(flipped, tile):
    black_adj = 0
    for adj_tile in get_adj(tile):
        if adj_tile in flipped:
            black_adj += 1

    return black_adj

for i in range(100):
    day = i + 1

    new_flipped = []

    # For each black tiles, we check it as 0 or > 2 black adjacent
    for tile in flipped:
        black_adj = get_black_adj(flipped, tile)
        if black_adj == 1 or black_adj == 2:
            if tile not in new_flipped:
                new_flipped.append(tile)

    # For any while tile with exactly 2 black tiles immediately adjacent
    for tile in flipped:
        for adj_tile in get_adj(tile):
            if adj_tile in flipped:
                continue

            if get_black_adj(flipped, adj_tile) == 2:
                if adj_tile not in new_flipped:
                    new_flipped.append(adj_tile)

    flipped = new_flipped

    # print(f"Day {day}: {len(flipped)}")

    if day == 100:
        break

print(f"Part 2: {len(flipped)}")
