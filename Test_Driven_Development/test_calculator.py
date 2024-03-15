#!/usr/bin/python3

import unittest
from calculator import Calculator

class TestCalculator(unittest.TestCase):
    @classmethod
    def setUp(self):
        self.add_1 = Calculator.add(89, 90)
        self.sub_1 = Calculator.sub(79, 20)
        self.mul_1 = Calculator.mul(78, 90)
        self.div_1 = Calculator.div(70, 10)
    
    def test_add(self):
        self.assertEqual(self.add_1, 179)
        self.assertNotEqual(self.add_1, 178)
    def test_sub(self):
        self.assertEqual(self.sub_1, 59)
        self.assertNotEqual(self.sub_1, 69)
    def test_mul(self):
        self.assertEqual(self.mul_1, 7020)
        self.assertNotEqual(self.mul_1, 7000)
    def test_div(self):
        self.assertEqual(self.div_1, 7)
        self.assertNotEqual(self.div_1, 6)

if __name__=="__main__":
    unittest.main()
