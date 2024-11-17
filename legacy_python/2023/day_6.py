from sys import argv
from collections import defaultdict
import re
from math import sqrt, ceil, floor
if __name__ == "__main__":


    durations = []
    best_distances = []

    with open(argv[1], "r") as input_:
        duration_line = input_.readline()
        distance_line = input_.readline()
        durations = [int(i) for i in re.findall("\d+", duration_line)]
        best_distances = [int(i) for  i in re.findall("\d+", distance_line)]

        p2_duration = int(re.search("\d+", duration_line.replace(" ", "")).group())
        p2_dist = int(re.search("\d+", distance_line.replace(" ", "")).group())
        print(p2_duration)
        print(p2_dist)
    
    total_p1 = 1
    for duration, best in zip(durations, best_distances):
        delta = duration**2 - 4*best
        sol_1 = -(-duration + sqrt(delta))/2
        sol_2 = -(-duration - sqrt(delta))/2
        
        total_p1 *= (ceil(sol_2 - 1) - floor(sol_1 + 1) + 1)


    delta = p2_duration**2 - 4*p2_dist
    sol_1 = -(-p2_duration + sqrt(delta))/2
    sol_2 = -(-p2_duration - sqrt(delta))/2
        
    total_p2= (ceil(sol_2 - 1) - floor(sol_1 + 1) + 1)

    print(total_p1)
    print(total_p2)
    