#!/usr/bin/env python3

import unittest
import main


class TestCalcMethods(unittest.TestCase):
    def test_part1(self):
        p1 = main.part1("./input.txt")
        self.assertEqual(p1, 917)

    def test_part2(self):
        p2 = main.part2("./input.txt")
        self.assertEqual(p2, 1649)


if __name__ == '__main__':
    unittest.main()
