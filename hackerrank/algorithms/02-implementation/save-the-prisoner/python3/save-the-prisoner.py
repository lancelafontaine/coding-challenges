#!/bin/python3

def save_the_prisoner(n,m,s):
    m = m % n
    s = s + m - 1
    if (s > n):
        s = s - n
    if (s < 1):
        s = s + n
    return s


if __name__ == '__main__':
    import sys
    t = int(input().strip())
    for _ in  range(t):
        n,m,s = [int(elem) for elem in input().split()]
        print(save_the_prisoner(n,m,s))
