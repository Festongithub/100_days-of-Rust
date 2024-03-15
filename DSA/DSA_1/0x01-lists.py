#!/usr/bin/python3

def product(my_list):
    for i in my_list:
        print("product is", i * i)

def length(my_list):
    print("Length is", len(my_list))

def right_elements(my_list):
    print("Left elements:", my_list[:])

def append_items(my_list, idx):
     my_list.extend(idx))
     return my_list


    
my_list = [90, 43, 34, 789, 34566, 900]
l = [900, 456, 321, 467, 8900]
def main():
    product(my_list)
    length(my_list)
    right_elements(my_list)
    append_items(my_list, l)
    
main()
