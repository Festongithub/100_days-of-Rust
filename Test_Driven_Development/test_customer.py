#!/usr/bin/python3
import unittest
from customer import Customer

class TestCustomer(unittest.TestCase):

    def setUp(self):
        self.customer_1 = Customer('John', 'Brad', 5000)
        self.customer_2 = Customer('Tina', 'Smith', 6000)
    def test_customer_email(self):
        self.assertEqual(self.customer_1.customer_email, 'John.Brad@email.com')
        self.assertEqual(self.customer_2.customer_email, 'Tina.Smith@email.com')
        self.assertNotEqual(self.customer_1.customer_email, 'John.brid@email.com')
    def test_customer_fullname(self):
        self.assertEqual(self.customer_1.customer_fullname, 'John Brad')
        self.assertEqual(self.customer_2.customer_fullname, 'Tina Smith')
    def test_apply_discount(self):
        self.customer_1.apply_discount()
        self.customer_2.apply_discount()

        self.assertEqual(self.customer_1.purchase, 4750)
        self.assertEqual(self.customer_2.purchase, 5700)
if __name__=="__main__":
    unittest.main()
