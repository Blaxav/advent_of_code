from sys import argv
import re

def decomp(n):
    L = dict()
    k = 2
    while n != 1:
        exp = 0
        while n % k == 0:
            n = n // k
            exp += 1
        if exp != 0:
            L[k] = exp
        k = k + 1
        
    return L

def _ppcm(a,b):
    Da = decomp(a)
    Db = decomp(b)
    p = 1
    for facteur , exposant in Da.items():
        if facteur in Db:
            exp = max(exposant , Db[facteur])
        else:
            exp = exposant
        
        p *= facteur**exp
        
    for facteur , exposant in Db.items():
        if facteur not in Da:
            p *= facteur**exposant
            
    return p

def ppcm(*args):
    L = list( args )
    if len(L) == 2:
        return _ppcm(L[0],L[1])
    else:
        n = len(L)
        i = 0
        A = []
        while i <= n-2:
            A.append( _ppcm( L[i] , L[i+1] ) )
            i += 2

        if n % 2 != 0:
            A.append( L[n-1] )
    
        return ppcm( *A ) 

def end_p1(elt):
    return elt == "ZZZ"

def end_p2(elt):
    return elt[-1] == "Z"

def go(start, end_getter, neighbz):
    id = 0
    iter = 0
    current = start
    while not end_getter(current):
        current = neighbz[current][seq[id]]
        iter += 1
        id += 1
        if id == len(seq):
            id = 0
    return iter

if __name__ == "__main__":

    seq = ""
    neighbz = {}

    with open(argv[1], "r") as input_:
        seq = input_.readline().rstrip()

        while line := input_.readline():
            if line == "\n":
                continue

            parsed = re.search("^(?P<init>[A-Z]{3}) = \((?P<left>[A-Z]{3}), (?P<right>[A-Z]{3})", line)

            neighbz[parsed.group("init")] = {
                "R": parsed.group("right"),
                "L": parsed.group("left"),
            }
    
    
    part_1 = go("AAA", end_p1, neighbz)
    print("Part 1: ", part_1)

    starts = []
    for key in neighbz:
        if key[-1] == "A":
            starts.append(key)

    ends = []
    for elt in starts:
        ends.append(go(elt, end_p2, neighbz))
    
    print(ends)
    print(ppcm(*ends))