#!/bin/python3

def divisible_sum_pairs(n, k, a):
    counter = 0
    for j in range(n):
        for i in range(j):
            if (float(a[i] + a[j])/k).is_integer():
                counter += 1
    return counter

if __name__ == '__main__':
    import sys
    n,k = input().strip().split(' ')
    n,k = [int(n),int(k)]
    a = [int(a_temp) for a_temp in input().strip().split(' ')]
    print(divisible_sum_pairs(n, k, a))

