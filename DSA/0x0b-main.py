#!/usr/bin/python3

import json

with open('Person.py', encoding="utf-8") as f:
    for line in f:
        print(line, end="")
        read_data = f.readlines()
        change = f.seek(0, 1)
        add = f.tell()
print(read_data)
print(change)
print(add)


with open('0x00-main.c', encoding=None) as f:
    print(f.name, f.encoding)
    for line in f:
        print(line, end="")

# Read data from a file 

a_file = open('Person.py', encoding="utf-8")

line_number = 0
for a_line in a_file:
    line_number += 1
    print('{:>4} {}'.format(line_number, a_line.strip()))
# chaneg the file position
print(a_file.seek(0))
print(a_file.tell())

# close the file
a_file.close()


with open('test.log', mode='w', encoding='utf-8') as test_file:
    test_file.write('Test succeeded')

with open('test.log', encoding='utf-8') as test_file:
    print(test_file.read())

# append data to the file

with open('test.log', mode='a', encoding='utf-8') as test_file:
    test_file.write('and again soldier we must conquer')

with open('test.log', encoding='utf-8') as test_file:
    print(list(test_file))
    # append data to the file

with open('test.log', mode='a', encoding='utf-8') as test_file:
        test_file.write('and again soldier we must conquer')

with open('test.log', encoding='utf-8') as test_file:
        print(list(test_file))

print(json.dump(['foo', {'bar' : ('baz', None, 1.0, 2)}]))
