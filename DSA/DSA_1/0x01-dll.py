#!/usr/bin/python3

class Node:
    def __init__(self, data, next=None):
        self.data = data
        self.next = next


    def push(self, new_data):
        new_node = Node(new_data)
        new_node.next = self.head
        self.head
