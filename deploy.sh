#!/bin/bash

echo "# 100_days-of-Rust" >> README.md
git init
git add README.md
git commit -m "first commit"
git branch -M main
git remote add origin https://github.com/Festongithub/100_days-of-Rust.git
git push -u origin main
