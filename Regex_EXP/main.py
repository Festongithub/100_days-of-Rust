#!/usr/bin/python3
import re

pattern = r'AB*'
text = ''

matches = re.findall(pattern, text)
print(matches)

nums = r'a+'
nums_text = 'abc aaa, aaaaa'
text_nums = re.findall(nums, nums_text)
print(text_nums)
