import re
from sys import argv
from collections import deque

def get_final_val_p1(seed, maps):
    current_value = seed
    final_value = current_value
    for map in maps:
        for key, val in map.items():
            if key[0] <= current_value < key[0] + key[1]:
                final_value = val + (current_value - key[0])

        current_value = final_value
    return final_value

def map_to_ranges(range_item, map) -> list[tuple]:
    """
    From a given range of elements (a,b) and a map of changes,
    returns the list of resulting ranges
    """
    remaining_ranges = [range_item]
    final_ranges = []
    for (map_st, map_rng), val in map.items():
        # Check if matches a remaining range
        range_to_create = []
        for (start, offset) in remaining_ranges:
            end = start + offset - 1

            if map_st >= start + offset or (map_st + map_rng - 1) < start:
                range_to_create.append((start, offset))
                continue

            
            # If there is an overlap, map it to a new range
            elif start <= map_st < start + offset:
                range_to_end = end - map_st + 1
                if map_st + map_rng <= start + offset:
                    range_to_end = map_rng

                final_ranges.append((val, range_to_end))
                
                if map_st > start:
                    # Part before the map range still need to be handled
                    range_to_create.append((start, map_st - start))
                if map_st + range_to_end < start + offset:
                    # Part after the map range still need to be handled
                    range_to_create.append((map_st + range_to_end, start + offset - map_st - range_to_end))

                continue
            
            # map starts before start of current range item
            else:
                if map_st + map_rng >= start + offset:
                    final_ranges.append(( val + start - map_st, offset))
                else:
                    map_end = map_st + map_rng - 1
                    final_ranges.append(( val + start - map_st, map_end - start))
                    range_to_create.append((map_end + 1, offset - (map_end - start)))

        

        remaining_ranges = [elt for elt  in range_to_create]
    return final_ranges + remaining_ranges



if __name__ == "__main__":

    all_maps = []
    input_seeds = []

    with open(argv[1], "r") as input_:
        for line in input_:
            if "seeds:" in line:    
                input_seeds = [int(i) for i in re.findall("\d+", line)]
                input_p2 = [[int(i) for i in result.split()] for result in re.findall("\d+ \d+", line)]
                continue

            # Creates next map
            if line == "\n":
                all_maps.append({})
                continue
            
            data = [int(i) for i in re.findall("\d+", line)]
            if not data:
                continue
            all_maps[-1][(data[1], data[2])] = data[0]
    
    finals = [] 
    for seed in input_seeds:
        finals.append(get_final_val_p1(seed, all_maps))

    final_p2 = []
    current_ranges = [elt for elt in input_p2]
    for map in all_maps:
        next_ranges = []
        for range_item in current_ranges:
            next_ranges.extend(map_to_ranges(range_item, map))
        current_ranges = [elt for elt in next_ranges]
        

    print("\nResults:")
    print(min(finals))
    print(min((elt[0] for elt in current_ranges))) 