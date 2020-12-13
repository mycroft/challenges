#!/usr/bin/env python

with open('input.txt') as f:
  content = f.read()
  f.close()

  lines = content.split('\n')

  i = 0
  j = 0
  k = 0

  while i < len(lines) - 3:
    j = i + 1
    while j < len(lines) - 2:
        k = j + 1
        while k < len(lines) - 1:
            if int(lines[i]) + int(lines[j]) + int(lines[k]) == 2020:
                print(int(lines[i]) * int(lines[j]) * int(lines[k]))

            k = k + 1

        j = j + 1

    i = i + 1
