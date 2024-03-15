#!/usr/bin/python3

import os

with open("mydata.txt", mode="w", encoding="utf-8") as myfile:
    myfile.write("Some one\nMore someone\nLess it\n")

with open("mydata.txt", encoding="utf-8") as myfile:
    print(myfile.read())

os.rename("mydata.txt", "mydata2.txt") 
os.mkdir("mydir")

print("Current directory: ", os.getcwd())
