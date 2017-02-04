#!/bin/python3
# Each spring, it doubles in height. Each summer, its height increases by 1 meter

def utopian_tree(n):
    height = 1
    for season in range(1,n+1):
        if season % 2 == 1:
            height = 2*height
        else:
            height += 1
    return height

if __name__ == '__main__':
    import sys
    t = int(input().strip())
    for a0 in range(t):
        n = int(input().strip())
        print(utopian_tree(n))
