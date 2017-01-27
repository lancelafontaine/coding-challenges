#!/bin/python3

import sys


n = int(input().strip())
arr = [int(arr_temp) for arr_temp in input().strip().split(' ')]

def plus_minus(arr):
    n = len(arr)
    (positive, negative, zero) = (0,0,0)
    for elem in arr:
        if elem > 0:
            positive += 1
        if elem < 0:
            negative += 1
        if elem == 0:
            zero += 1
    return (positive/n, negative/n, zero/n)

(positive, negative, zero) = plus_minus(arr)
print("{:.6f}".format(positive))
print("{:.6f}".format(negative))
print("{:.6f}".format(zero))

