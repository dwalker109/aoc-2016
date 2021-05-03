#!/usr/bin/env python3

import unittest
import main


class TestCalcMethods(unittest.TestCase):
    def testPart1(self):
        (p1, _) = main.calc("./input_test.txt")
        self.assertEqual(p1, 8)

    def testPart2(self):
        (_, p2) = main.calc("./input_test.txt")
        self.assertEqual(p2, 4)


if __name__ == '__main__':
    unittest.main()
