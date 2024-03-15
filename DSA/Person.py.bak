#!/usr/bin/python3

"""
A simple class for Person
"""
class Person:
    bmr = 0
    def __init__(self, f_name, l_name,  age, weight, height):
        self.f_name = f_name
        self.l_name = l_name
        self.age = age
        self.weight = weight
        self.height = height


    def email(self):
        print("{}.{}".format(self.f_name, self.l_name))

        
    def get_age(self):
        print("age is {}".format(self.age))

    def check_weight(self):
        print("weight is {}".format(self.weight))

    def height(self):
        print("{} is {} meters high".format(self.name, self.height))

    def bmr(self):
        self.bmr = self.weight ** 2 / self.height
        print("{}".format(self.bmr))

        
class Student (Person):

    def __init__(self, email, year):
        self.email = email
        self.year = year

    
