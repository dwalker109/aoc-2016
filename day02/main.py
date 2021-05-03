#!/usr/bin/env python3

def main(path):
    p1 = get_code_9x9(path)
    p2 = get_code_batshit(path)

    print("Part 1: {0}".format(p1))
    print("Part 2: {0}".format(p2))


def get_code_9x9(path):
    curr = 5
    result = []

    moves = {
        "U": lambda x: -3 if x > 3 else 0,
        "D": lambda x: +3 if x < 7 else 0,
        "L": lambda x: -1 if x not in [1, 4, 7] else 0,
        "R": lambda x: +1 if x not in [3, 6, 9] else 0
    }

    instr = from_input(path)

    for line in instr:
        for move in line:
            curr = curr + moves[move](curr)

        result.append(curr)

    return "".join(map(str, result))


def get_code_batshit(path):
    curr = (0, 2)
    result = []

    pad = {
        (2, 0): "1",

        (1, 1): "2",
        (2, 1): "3",
        (3, 1): "4",

        (0, 2): "5",
        (1, 2): "6",
        (2, 2): "7",
        (3, 2): "8",
        (4, 2): "9",

        (1, 3): "A",
        (2, 3): "B",
        (3, 3): "C",

        (2, 4): "D",
    }

    moves = {
        "U": lambda x, y: (x, y - 1),
        "D": lambda x, y: (x, y + 1),
        "L": lambda x, y: (x - 1, y),
        "R": lambda x, y: (x + 1, y),
    }

    instr = from_input(path)

    for line in instr:
        for move in line:
            x, y = curr
            next = moves[move](x, y)
            if next in pad:
                curr = next

        result.append(pad[curr])

    return "".join(result)


def from_input(path):
    instr = []
    with open(path, encoding="utf-8") as f:
        for line in f:
            instr.append(line.strip("\n"))

    return instr


if __name__ == "__main__":
    main("./input.txt")
