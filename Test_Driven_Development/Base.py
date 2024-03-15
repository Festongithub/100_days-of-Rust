#!/usr/bin/python3
import cmd, sys
import turtle
from turtle import *

class Base(cmd.Cmd):
    intro = 'Welcome to the turtle shell'
    prompt = '$'
    file = None

    def do_forward(self, arg):
        forward(*parse(arg))

    def do_right(self, arg):
        right(*parse(arg))
        
    def do_left(self, arg):
        left(*parse(arg))
        
    def do_goto(self, arg):
        goto(*parse(arg))
        
    def do_home(self,arg):
        home()
        
    def do_Circle(self,arg):
        Circle(*parse(arg))
        
    def do_position(self, arg):
        print("Current position is %d %d\n" % position())

    def do_color(self, arg):
        color(arg.lower())
        
    def do_heading(self,arg):
        print("Cuurent Heading is %d\n", (heading(), ))

    def do_undo(self, arg):
        "Undo (repeatedly) the last turtle action(s): UNDO"
    def do_reset(self,arg):
        'clear screen and return  turtle to center: RESET'
        reset()
        
    def do_bye(self, arg):
        printf('Thank you for using turtle')
        self.close()
        bye()
        return True
    #-----record and playback-----
    def do_record(self, arg):
        self.file = open(arg, 'w')
        
    def do_playback(self,arg):
        self.close()
        with open(arg) as f:
            self.cmdqueue.extend(f.read().splitlines())
            
    def precmd(self, line):
        line = line.lower()
        if self.file and 'playback' not in line:
            print(line, file=self.file)

    def close(self):
        if self.file:
            self.file.close()
            self.file = None
def parse(arg):
    return tuple(map(int, arg.split()))

if __name__ == "__main__":
    Base().cmdloop()
