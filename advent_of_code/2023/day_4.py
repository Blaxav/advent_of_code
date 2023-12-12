from sys import argv
from collections import defaultdict

if __name__ == "__main__":

    total_p1 = 0
    copies = defaultdict(lambda: 1)
    total_games = 0
    with open(argv[1], "r") as input_:
        for id, card in enumerate(input_):
            data = card.split(":", 1)[1]
            winning = set(int(elt) for elt in data.split("|")[0].split())
            drawn = set(int(elt) for elt in data.split("|")[1].split())
            if won := winning & drawn:
                total_p1 += 2**(len(won) - 1)
                for offset in range(len(won)):
                    copies[id + offset + 1] += copies[id]
            total_games = id + 1
    
    # Part 1: 
    print(total_p1)
    # Part 2:
    # Adding the entries that have never been won counting as 1
    never_won = total_games - len(copies)
    print(sum(copies.values()) + never_won)
    