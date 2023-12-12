import re
from sys import argv


match_to_number = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
}

if __name__ == "__main__":
    total = 0
    with open(argv[1], "r") as input_:
        for line in input_:
            matches = []
            for i in range(len(line)):
                if res := re.match(
                    f"(\d|one|two|three|four|five|six|seven|eight|nine|zero)", line[i:]
                ):
                    matches.append(res.group(0))
            for i in (0, -1):
                if matches[i] in match_to_number:
                    matches[i] = match_to_number[matches[i]]
            total += int(matches[0] + matches[-1])
    print(total)
