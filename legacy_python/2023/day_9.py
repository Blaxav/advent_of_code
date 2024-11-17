import re
from sys import argv

def next_p1(game):
    ends = []
    begins = []
    current = [i for i in game]
    ends.append(current[-1])
    begins.append(current[0])
    while any(elt != 0 for elt in current):
        current = [current[i+1] - current[i] for i in range(len(current) - 1)]
        if current:
            ends.append(current[-1])
            begins.append(current[0])
    return sum(ends), sum([(-1)**i * elt for i, elt in enumerate(begins)])



if __name__ == "__main__":

    res = []
    p_2 = []
    with open(argv[1], "r") as input_:
        for line in input_:
            draws = [int(i) for i in re.findall("-?\d+", line)]
            res_p1, res_p2 = next_p1(draws)
            res.append(res_p1)
            p_2.append(res_p2)
    print(sum(res))

    print()
    print(sum(p_2))