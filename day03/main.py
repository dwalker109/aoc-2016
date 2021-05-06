#!/usr/bin/env python3

def main(path):

    p1 = part1(path)
    p2 = part2(path)

    print("Part 1: {0}".format(p1))
    print("Part 2: {0}".format(p2))


def part1(path):
    dims = get_input_rows(path)

    return calc_possible_triangles(dims)


def part2(path):
    dims = get_input_cols(path)

    return calc_possible_triangles(dims)


def calc_possible_triangles(dims):
    possibles = 0

    for triangle in dims:
        a, b, c = triangle
        if a+b > c and a+c > b and b+c > a:
            possibles += 1

    return possibles


def get_input_rows(path):
    dims = []
    with open(path, encoding="utf-8") as f:
        for l in f:
            curr = list(map(int, l.split()))
            assert(len(curr) == 3)
            dims.append(curr)

    return dims


def get_input_cols(path):
    lines = []
    with open(path, encoding="utf-8") as f:
        for l in f:
            curr = list(map(int, l.split()))
            assert(len(curr) == 3)
            lines.append(curr)

    dims = []
    for x in range(3):
        for y in range(0, len(lines), 3):
            dims.append([lines[y][x], lines[y+1][x], lines[y+2][x]])

    return dims


if __name__ == "__main__":
    main("./input.txt")
