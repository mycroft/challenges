#!/usr/bin/env python

fd = open('input.txt')
lines = fd.read().split('\n')
fd.close()

dep = 1000509
buses = [17, 37, 739, 29, 13, 23, 971, 41, 19]

print("=== part 1 ===")

for bus in buses:
    q, r = divmod(dep, bus)

    next_dep = (q + 1) * bus
    wait_time = next_dep - dep
    result = bus * wait_time

    print(bus, next_dep, wait_time, result)

start_ts = int(lines[0])
schedule = lines[1]

print(start_ts, schedule)


print()
print("=== part 2 ===")


# schedule = '17,x,13,19'

buses = []

for idx, bus in enumerate(schedule.split(',')):
    if bus == 'x':
        continue
    buses.append((int(bus), idx))


period = 1
ts = 0

def get_next(period, ts, bus_period, bus_delay):
    print(f'get_next({period}, {ts}, {bus_period})')

    while True:
        if (ts + bus_delay) % bus_period == 0:
            return ts

        ts += period

for bus in buses:
    print(f'bus:{bus}')
    ts = get_next(period, ts, bus[0], bus[1])

    period *= bus[0]

print(f'found ts = {ts}')

