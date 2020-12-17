#!/usr/bin/env python

fd = open('input.txt')
lines = fd.read().split('\n')
fd.close()

instructions = []

for line in lines:
    if line == '':
        break
    ins, val = line.split(' ')
    val = int(val)

    instructions.append((ins, val))



def test(instructions):
    global_acc = 0
    global_step = 0

    visited = []

    while True:
        if global_step == len(instructions):
            print("program halted")
            break

        current_ins = instructions[global_step]

        if global_step in visited:
            break
        visited.append(global_step)

        if current_ins[0] == 'nop':
            global_step += 1
            continue

        if current_ins[0] == 'jmp':
            global_step += current_ins[1]
            continue

        if current_ins[0] == 'acc':
            global_acc += current_ins[1]
            global_step += 1
            continue

    if global_step == len(instructions):
        print('acc', global_acc, 'step', global_step)



orig_instructions = instructions

for idx in range(len(instructions)):
    if instructions[idx][0] == 'nop':
        new_instruction = instructions[idx]
        new_instruction = ('jmp', instructions[idx][1])
    if instructions[idx][0] == 'jmp':
        new_instruction = instructions[idx]
        new_instruction = ('nop', instructions[idx][1])

    # print(instructions[:idx] + [new_instruction] + instructions[idx+1:])

    try:
        test(instructions[:idx] + [new_instruction] + instructions[idx+1:])
    except Exception as e:
        print(e)
