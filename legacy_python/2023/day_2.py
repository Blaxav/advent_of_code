import re
from sys import argv


def game_result_part_1(game: str) -> int:
    id = int(re.search("Game (?P<game_id>\d+):", game).group("game_id"))
    rounds = game.split(":")[1].split(";")
    invalid = False
    for round in rounds:
        revealed = re.finditer("(?P<number>\d+) (?P<color>(red|green|blue))", round)
        for elt in revealed:
            color = elt.group("color")
            number = int(elt.group("number"))
            if number > cubes_by_color[color]:
                invalid = True
    if not invalid:
        return id
    return 0


def game_result_part_2(game: str) -> int:
    value = 1
    for color in ("red", "green", "blue"):
        value *= max(int(i) for i in re.findall(f"(?<= )\d+(?= {color})", game))
    return value


if __name__ == "__main__":
    cubes_by_color = {"red": 12, "green": 13, "blue": 14}

    with open(argv[1], "r") as input_:
        # Part 1
        part_1 = 0
        part_2 = 0
        for game in input_:
            part_1 += game_result_part_1(game)
            part_2 += game_result_part_2(game)
        print("Part 1: ", part_1)
        print("Part 2: ", part_2)
