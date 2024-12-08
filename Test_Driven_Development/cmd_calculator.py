#!/usr/bin/python3
import cmd

class Calculator(cmd.Cmd):
    intro = 'Welcome to the Maths calculator'
    prompt = '$1'

    def do_add(a, b):
        'Adds two integers and finds their sum'
        a = int(input("Enter value of a: "))
        b = int(input("Enter value of b: "))
        print("Result is {}".format(a + b))
        
    def do_sub(a, b):
        'Finds the difference of two integers'
        a = int(input("Enter value of a: "))
        b = int(input("Enter value of a: "))
        print("Subtractioo is {}".format(a - b))
              
    def do_div(a, b):
        'Division of a and b'
        a = int(input("Enter value of a: "))
        b = int(input("Enter value of b: "))
        if b == 0:
            raise ZeroDivisionError("Division error")
        else:
            print("Division of {} / {}  = {}".format(a, b, a/b))
    def do_EOF(self, line):
        return True
        
if __name__ =="__main__":
    Calculator().cmdloop()
