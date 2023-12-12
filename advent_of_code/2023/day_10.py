import re
from sys import argv
from collections import deque

def neighbors_of_elt(i,j,char):
    match elt:
        case "S":
            return ["start"]
        case "-":
            return [(i,j-1), (i,j+1)]
        case "|":
            return [(i-1,j), (i+1,j)]
        case "F":
            return [(i+1,j), (i, j+1)]
        case "L":
            return [(i-1,j), (i, j+1)]
        case "J": 
            return [(i-1,j), (i, j-1)]
        case "7":
            return [(i+1,j), (i, j-1)]
        case _:
            return []
        

def get_next_element(path, nodes):
    if len(path) == 1:
        # Init, finding possible starting paths
        i,j = path[0]
        for k in [-1, 0, 1]:
            for p in [-1, 0, 1]:
                if (i,j) in nodes.get((i+k, j+p), []):
                    return (i+k, j+p)
        # Should not get here
        raise StopIteration
    last = path[-1]
    previous = path[-2]
    
    return next(elt for elt in nodes[last] if elt != previous)

def get_val(i, elt, path):
    match elt:
        case "L":
            if path[i-1][1] - path[i][1] > 0:
                return 1
            else:
                return -1
        case "F":
            if path[i-1][0] - path[i][0] > 0:
                return 1
            else:
                return -1
        case "7":
            if path[i][1] - path[i-1][1] > 0:
                return 1
            else:
                return -1
        case "J":
            if path[i][0] - path[i-1][0] > 0:
                return 1
            else:
                return -1
    return 0
if __name__ == "__main__":

    nodes = {}
    figs = {}

    all_paths = deque()
    start = "" # Remember start to know when back to it

    with open(argv[1], "r") as input_:
        num_line = 0
        for line in input_:
            for num_col, elt in enumerate(line.rstrip()):
                nodes[(num_line, num_col)] = neighbors_of_elt(num_line, num_col, elt)
                figs[(num_line, num_col)] = elt
                if elt == "S":
                    start = (num_line, num_col)
            num_line += 1
                    
    path = [start]

    while path[-1] != "start":
        path.append(get_next_element(path, nodes))
        
    print(len(path) - 1)
    print(int((len(path) - 1)/2))
    

    cnt = 0
    min_line = min([elt[0] for elt in path])
    min_col= min([elt[1] for elt in path])
    max_line = max([elt[0] for elt in path])
    max_col= max([elt[1] for elt in path])
    for i in range(min_line, max_line + 1):
        for j in range(min_col, max_col + 1):



    all_vals = []

    for i, elt in enumerate(path[:-1]):
        current_elt = figs[elt]
        print(current_elt)
        if elt == "S":
            pass
        all_vals.append(get_val(i, current_elt, path))
    print(all_vals)
    print(sum(all_vals) + 1)