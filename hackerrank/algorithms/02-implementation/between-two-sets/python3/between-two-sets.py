#!/bin/python3

def between_two_sets(a, b):
    counter = 0
    for i in range(max(a), min(b)+1):
        should_increment = True

        for num in a:
            if not (float(i)/num).is_integer():
                should_increment = False

        for num in b:
            if not (float(num)/i).is_integer():
                should_increment = False

        if should_increment:
            counter += 1

    return counter

if __name__ == '__main__':
    import sys
    n,m = input().strip().split(' ')
    n,m = [int(n),int(m)]
    a = [int(a_temp) for a_temp in input().strip().split(' ')]
    b = [int(b_temp) for b_temp in input().strip().split(' ')]
    print(between_two_sets(a, b))

