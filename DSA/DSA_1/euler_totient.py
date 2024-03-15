#!/usr/bin/python3

def euler_totient(a, b, c):
    a = (a * (1 - 1/b) * ( 1 - 1/c))
def main():
    a = int(input("Enter conumber: "))
    b = int(input("Enter b: "))
    c = int(input("Enter c:"))
    print("result is ",  euler_totient(a, b, c))
main()
