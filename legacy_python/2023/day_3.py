import re
from sys import argv
from collections import defaultdict

def is_adjacent_to_symb(coordinates: tuple[int, int], chars: list) -> bool:
    pass

class Number:
    def __init__(self, num_line, span, value):
        self.num_line = num_line
        self.span = tuple(int(elt) for elt in span)
        self.value = int(value)
    
    def is_adjacent_to_symb(self, chars: list[tuple[int, int]], ratios: defaultdict[list]):
        is_ok = False
        for index,symb in enumerate(chars):
            if symb[0] > self.num_line + 1 or symb[0] < self.num_line - 1:
                continue
            if self.span[0] -1 <= symb[1] < self.span[1] + 1:
                if symb[2] == "*":
                    ratios[(symb[0], symb[1])].append(self.value)
                is_ok = True
        return is_ok

if __name__ == "__main__":

    numbers = []
    chars = []
    ratios = defaultdict(list)

    with open(argv[1], "r") as input_:
        for num_line, line in enumerate(input_):
            for catch in re.finditer("\d+", line.rstrip()):
                numbers.append(Number(num_line=num_line, span=catch.span(), value=catch.group(0)))

            for num_col, elt in enumerate(line.rstrip()):
                if not elt.isdigit() and elt != ".":
                    chars.append((int(num_line), int(num_col), elt))
    total = 0
    for num  in numbers:
        if num.is_adjacent_to_symb(chars, ratios):
            total += num.value
    total_p2 = 0
    for elt in ratios.values():
        if len(elt) == 2:
            total_p2 += elt[0]*elt[1]
    print(total)
    print(total_p2)
