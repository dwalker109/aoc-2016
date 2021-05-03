#!/usr/bin/env python3

import unittest
import main


class TestCalcMethods(unittest.TestCase):
    def test_part1(self):
        p1 = main.get_code_9x9("./input_test.txt")
        self.assertEqual(p1, "1985")

    def test_part2(self):
        p2 = main.get_code_batshit("./input_test.txt")
        self.assertEqual(p2, "5DB3")


if __name__ == '__main__':
    unittest.main()
