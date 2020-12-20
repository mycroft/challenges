#!/usr/bin/env python

import re
import math
import sys

fd = open('input.txt')
groups = fd.read().split("\n\n")
fd.close()

print("part #1")

tiles = {}


for group in groups:
    tile = []
    tilenum = 0

    for line in group.splitlines():
        m = re.match(r'^Tile (\d+):$', line)
        if m:
            tilenum = int(m.groups()[0])
            continue

        if line != '':
            tile.append(list(line))

    tiles[tilenum] = tile

# returns the same tile, flipped <->
def flip(tile):
    new_tile = []

    for tile_row in tile:
        new_tile_row = tile_row.copy()
        new_tile_row.reverse()
        new_tile.append(new_tile_row)

    return new_tile

# rotates the time
def rotate(tile):
    tuples = zip(*tile[::-1])
    return list([list(elem) for elem in tuples])

# returns possible tiles for a given tile
def possibles(tile):
    possibles = []

    flipped = flip(tile)

    for n in range(4):
        flipped = rotate(flipped)
        possibles.append(flipped)
        tile = rotate(tile)
        possibles.append(tile)

    return possibles

# check that putting tile in grid at i, j is possible or not
def is_possible(tile, grid, i, j):
    co = 0

    # check at left
    if i > 0 and grid[i-1][j] != None:
        co += 1
        for k in range(10):
            if tile[k][0] != grid[i-1][j][k][9]:
                # print('v', list(tile[k][0] for k in range(10)), '!=', list(grid[i-1][j][k][9] for k in range(10)))
                return False

    # check at right
#    if i < 9 and grid[i+1][j] != None:
#        co += 1
#        for k in range(10):
#            if tile[k][9] != grid[i+1][j][k][0]:
#                return False

    # check at up
    if j >  0 and grid[i][j-1] != None:
        co += 1
        for k in range(10):
            if tile[0][k] != grid[i][j-1][9][k]:
                # print('h', tile[0], '!=', grid[i][j-1][9])
                return False

    # check at down
#    if j < 9 and grid[i][j + 1] != None:
#        co += 1
#        for k in range(10):
#            if tile[9][k] != grid[i][j+1][0][k]:
#                return False

    if i != 0 and j != 0:
        return co >= 1

    return True


def do_try():
    if not stack:
        return True

    (i, j) = stack.pop()

    # print(f"adding at {i} {j}")

    for k in list(possible_tiles):
        # print(len(stack), 'k', k)
        possibles_values = possible_tiles[k]
        del possible_tiles[k]

        for possible_tile in possibles_values:
            # print("testing", k)
            if is_possible(possible_tile, grid, i, j):
                grid[i][j] = possible_tile
                grid_v[i][j] = k

                # print(grid_v, i, j, 'added', k)
                # print(f'Adding {k} at {i}x{j}')
                if do_try():
                    return True

                grid[i][j] = None
                grid_v[i][j] = 0

        possible_tiles[k] = possibles_values

    stack.append((i, j))


possible_tiles = {}
for k in tiles:
    possible_tiles[k] = possibles(tiles[k])

grid_size = int(math.sqrt(len(tiles)))

grid = []
grid_v = []

# We're using a stack: we want 0,0 on top, then 0, 1... 1, 0, ... 12, 12.
stack = list(reversed(list((r, c) for c in range(grid_size) for r in range(grid_size))))

for i in range(grid_size):
    grid.append([None] * grid_size)
    grid_v.append([0] * grid_size)

do_try()

print(grid_v[0][0], grid_v[0][-1], grid_v[-1][0], grid_v[-1][-1])
print(grid_v[0][0] * grid_v[0][-1] * grid_v[-1][0] * grid_v[-1][-1])

print("part #2")

new_grid = []

for i in range(len(grid)):
    new_grid_row = []
    for j in range(len(grid)):
        new_tile = []
        current_tile = grid[i][j]
        del current_tile[9]
        del current_tile[0]

        for row in current_tile:
            new_tile.append(row[1:-1])

        new_grid_row.append(new_tile)

    new_grid.append(new_grid_row)


grid = new_grid

mega_grid = []
for i in range(len(grid) * len(grid[0][0])): # each row
    mega_line = []
    for j in range(len(grid) * len(grid[0][0])): # each column
        tile = grid[int(i/8)][int(j/8)]

        mega_line.append(tile[j%8][i%8])

    mega_grid.append(mega_line)

# def dump_grid(grid):
#     for l in range(len(grid)):
#         print(''.join(grid[l]))

all_grids = possibles(mega_grid)

count = 0

for i in range(len(mega_grid)):
    for j in range(len(mega_grid)):
        if mega_grid[i][j] == '#':
            count += 1

seaquest = ['                  # ', '#    ##    ##    ###', ' #  #  #  #  #  #   ']

num = 0

for mega_grid in all_grids:
    for i in range(len(mega_grid[0]) - len(seaquest[0])):
        for j in range(len(mega_grid) - len(seaquest)):

            found = True
            for k in range(len(seaquest[0])):
                for l in range(3):
                    if seaquest[l][k] == ' ':
                        continue

                    if seaquest[l][k] == '#' and mega_grid[j + l][i + k] != '#':
                        found = False

                    if not found:
                        break

                if not found:
                    break

            if found:
                # print(f"found at {i} {j}")
                num += 1

print(count - num * 15)
