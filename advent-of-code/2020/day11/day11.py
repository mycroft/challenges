#!/usr/bin/env python

import sys

fd = open('input.txt')
lines = fd.read().split('\n')
fd.close()

seats = []

for line in lines:
    if line == '':
        break
    seats.append(list(line))

orig_seats = seats

def get_adjacent(seats, i, j):
    busy = 0
    free = 0

    for i2 in range(max(i-1, 0), min(i+2, len(seats))):
        for j2 in range(max(j-1, 0), min(j+2, len(seats[0]))):
            if i == i2 and j == j2:
                continue

            if seats[i2][j2] == '#':
                busy += 1
            if seats[i2][j2] == 'L':
                free += 1

    return busy, free

def get_view(seats, i, j):
    directions = [
    #    i, j
        [-1, 1],
        [0, 1],
        [1, 1],
        [-1, 0],
        [1, 0],
        [-1,-1],
        [0, -1],
        [1, -1]
    ]

    busy = 0
    free = 0

    for direction in directions:
        it = 1
        while True:
            if it * direction[0] + i < 0 or it * direction[0] + i >= len(seats):
                break
            if it * direction[1] + j < 0 or it * direction[1] + j >= len(seats[0]):
                break

            if seats[it * direction[0] + i][it * direction[1] + j] == '#':
                busy += 1
                break

            if seats[it * direction[0] + i][it * direction[1] + j] == 'L':
                free += 1
                break

            it += 1

    return busy, free


def get_view2(arr, x, y):
    to = 0
    dirs = [
        (-1,0),
        (1,0),
        (-1,1),
        (1,1),
        (-1,-1),
        (1,-1),
        (0,-1),
        (0,1),
    ]
    for coor in dirs:
        i = 1
        while i <= max(len(arr)+1,len(arr[0])+1):
            currx = x + i * coor[0]
            curry = y + i * coor[1]

            if valid_location(currx,curry,arr):
                if is_seat(arr[currx][curry]):
                    if arr[currx][curry] == '#':
                        to+=1
                    break
            else:
                break
            i+=1

    return to

def is_seat(c):
    return c != '.'

def valid_location(x, y, arr):
    return x >= 0 and y >= 0 and x < len(arr) and y < len(arr[0])

def get_stats(seats):
    busy, free, total = 0, 0, 0

    for i in range(len(seats)):
        for j in range(len(seats[0])):
            total += 1
            if seats[i][j] == '#':
                busy += 1
            elif seats[i][j] == 'L':
                free += 1

    return busy, free, total

iterations = 100

while iterations != 0:
    new_seats = []
    for i in range(len(seats)):
        new_row = []
        for j in range(len(seats[0])):
            if seats[i][j] == '.':
                new_row.append('.')
                continue

            busy, free = get_adjacent(seats, i, j)
            # print(i, j, seats[i][j], busy, free)

            if busy == 0:
                new_row.append('#')
                continue

            if busy >= 4:
                new_row.append('L')
                continue

            new_row.append(seats[i][j])

        new_seats.append(new_row)

    seats = new_seats
    busy, free, total = get_stats(seats)

    print('busy:', busy, ' free:', free, ' total:', total)

    iterations -= 1


print(get_view(seats, 0, 0))

iterations = 100
seats = orig_seats

while iterations != 0:
    new_seats = []
    for i in range(len(seats)):
        new_row = []
        for j in range(len(seats[0])):
            if seats[i][j] == '.':
                new_row.append('.')
                continue

            busy, free = get_view(seats, i, j)
            # busy = get_view2(seats, i, j)
            # print(i, j, seats[i][j], busy, free)

            if busy == 0:
                new_row.append('#')
                continue

            if busy >= 5:
                new_row.append('L')
                continue

            new_row.append(seats[i][j])

        new_seats.append(new_row)

    seats = new_seats
    busy, free, total = get_stats(seats)

    print('busy:', busy, ' free:', free, ' total:', total)

    iterations -= 1
