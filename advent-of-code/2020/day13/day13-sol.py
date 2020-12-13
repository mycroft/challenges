#!/usr/bin/env python

fd = open('input.txt')
lines = fd.read().split('\n')
fd.close()


from functools import reduce
def chinese_remainder(n, a):
    sum = 0
    prod = reduce(lambda a, b: a*b, n)
    for n_i, a_i in zip(n, a):
        p = prod // n_i
        sum += a_i * mul_inv(p, n_i) * p
    return sum % prod


def mul_inv(a, b):
    b0 = b
    x0, x1 = 0, 1
    if b == 1: return 1
    while a > 1:
        q = a // b
        a, b = b, a%b
        x0, x1 = x1 - q * x0, x0
    if x1 < 0: x1 += b0
    return x1



dep = 1000509
buses = [17, 37, 739, 29, 13, 23, 971, 41, 19]

for bus in buses:
    q, r = divmod(dep, bus)

    next_dep = (q + 1) * bus
    wait_time = next_dep - dep
    result = bus * wait_time

    print(bus, next_dep, wait_time, result)

def next(current_ts, num):
    q, r = divmod(current_ts, num)

    next_dep = (q + 1) * num

    return next_dep

start_ts = int(lines[0])
schedule = lines[1]

buses = []

bigbus = 0

a, n = [], []
schedule = schedule.split(',')

for i, x in enumerate(schedule):
    if x == 'x':
        continue
    x = int(x)

    a.append(x - i % x)
    n.append(x)

print(a, n)

print(chinese_remainder(n, a))




# while True:
#     next_dep = next(next_dep, bigbus)
#     real_t = next_dep - bigts

#     found = True

#     for bus_spec in buses:
#         next_dep_sub = next(real_t + bus_spec[1] - 1, bus_spec[0])

#         print('>', 
#               real_t, bus_spec,
#               'got:', next_dep_sub,
#               'want:', real_t + bus_spec[1],
#                next_dep_sub != real_t + bus_spec[1])

#         if next_dep_sub != real_t + bus_spec[1]:
#             found = False
#             break

#     if found:
#         print('found valid next departure time:', real_t)
#         break


