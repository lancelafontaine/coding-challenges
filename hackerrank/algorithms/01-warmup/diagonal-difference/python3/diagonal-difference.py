#!/bin/python3

import sys


n = int(input().strip())
a = []
for a_i in range(n):
    a_t = [int(a_temp) for a_temp in input().strip().split(' ')]
    a.append(a_t)

def diagonal_difference(a):
    (diagonal_sum1, diagonal_sum2) = (0,0)
    for i in range(len(a)):
        diagonal_sum1 += a[i][i]
        diagonal_sum2 += a[i][n-1-i]
    return abs(diagonal_sum1 - diagonal_sum2)

print(diagonal_difference(a))
