#!/usr/bin/python3

import json
text_file = open("actor.txt", "r", encoding="utf-8")
movie = json.load(text_file)
print(movie)
