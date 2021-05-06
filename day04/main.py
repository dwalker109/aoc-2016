#!/usr/bin/env python3
from itertools import groupby


def main(path):

    p1 = part1(path)
    p2 = part2(path)

    print("Part 1: {0}".format(p1))
    print("Part 2: {0}".format(p2))


def part1(path):
    rooms = get_input(path)
    real = decipher(rooms)
    return real


def part2(path):
    return 0


def get_input(path):
    rooms = []
    with open(path, encoding="utf-8") as f:
        for l in f:
            parts = l.split("-")
            id, checksum = parts.pop(-1).strip("]\n").split("[")
            rooms.append({
                "name": sorted("".join(parts)), "id": int(id), "checksum": checksum
            })

    return rooms


def decipher(rooms):
    summed = 0

    for room in rooms:
        letters = [list(g) for k, g in groupby(room["name"])]
        letters = sorted(letters, key=lambda grp: grp[0], reverse=False)
        letters = sorted(letters, key=lambda grp: len(grp), reverse=True)
        checksum = "".join([l[0] for l in letters[:5]])
        if checksum == room["checksum"]:
            summed += room["id"]

    return summed


if __name__ == "__main__":
    main("./input.txt")
