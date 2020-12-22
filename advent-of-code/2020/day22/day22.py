#!/usr/bin/env python

import copy

fd = open('input.txt')
players = fd.read().split('\n\n')
fd.close()

decks = []

for player in players:
    deck = []
    for line in player.split('\n'):
        if not line.isnumeric():
            continue

        deck.append(int(line))

    decks.append(deck)

def play(decks, game_type = 1):
    # Before either player deals a card, if there was a previous round in
    # this game that had exactly the same cards in the same order in the
    # same players' decks, the game instantly ends in a win for player 1.

    history = set()

    while len(decks[0]) > 0 and len(decks[1]) > 0:
        round_config = (tuple(decks[0]), tuple(decks[1]))
        if round_config in history:
            return 0
        else:
            history.add(round_config)

        c1 = decks[0][0]
        del decks[0][0]

        c2 = decks[1][0]
        del decks[1][0]

        winner = None

        if game_type == 2 and (len(decks[0]) >= c1 and len(decks[1]) >= c2):
            new_decks = copy.deepcopy(decks)
            new_decks[0] = new_decks[0][:c1]
            new_decks[1] = new_decks[1][:c2]
            winner = play(new_decks, game_type)

        if winner == 0 or (winner == None and c1 > c2):
            decks[0].append(c1)
            decks[0].append(c2)
        else:
            decks[1].append(c2)
            decks[1].append(c1)

    if len(decks[1]) > 0:
        return 1

    return 0

def compute_score(deck):
    score = 0
    i = len(deck)
    while i > 0:
        score += i * deck[len(deck) - i]
        i -= 1

    return score

orig_decks = copy.deepcopy(decks)
print(compute_score(decks[play(decks, 1)]))

decks = orig_decks
print(compute_score(decks[play(decks, 2)]))
