#!/usr/bin/env python3

def main(path):
    p1, p2 = calc(path)
    print("Part 1: {0}".format(p1))
    print("Part 2: {0}".format(p2))


def calc(path):
    loc = (0, 0)
    bearings = (
        lambda: (loc[0], loc[1] + 1),  # N
        lambda: (loc[0] + 1, loc[1]),  # E
        lambda: (loc[0], loc[1] - 1),  # S
        lambda: (loc[0] - 1, loc[1]),  # W
    )
    bearing = 0
    hist = {}
    twice = ()

    with open(path, encoding="utf-8") as f:
        steps = f.readline().strip("\n").split(", ")
        for s in steps:
            d, q = s[0], int(s[1:])

            if d == "L":
                bearing = (bearing - 1) % len(bearings)
            elif d == "R":
                bearing = (bearing + 1) % len(bearings)

            for i in range(q):
                loc = bearings[bearing]()
                hist.update({loc: hist.get(loc, 0) + 1})
                if twice == () and hist.get(loc) > 1:
                    twice = loc

    return (
        abs(loc[0]) + abs(loc[1]),
        abs(twice[0]) + abs(twice[1])
    )


if __name__ == "__main__":
    main("input.txt")
