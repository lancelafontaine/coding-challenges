#!/bin/python3

def angry_professor(n,k,a):
    early_students = [elem for elem in a if elem <= 0]
    if len(early_students) < k:
        return 'YES'
    return 'NO'

if __name__ == '__main__':
    import sys
    t = int(input().strip())
    for a0 in range(t):
        n,k = input().strip().split(' ')
        n,k = [int(n),int(k)]
        a = [int(a_temp) for a_temp in input().strip().split(' ')]
        print(angry_professor(n,k,a))
