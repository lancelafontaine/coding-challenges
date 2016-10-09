#!/bin/python3

import sys

a0,a1,a2 = input().strip().split(' ')
a0,a1,a2 = [int(a0),int(a1),int(a2)]
b0,b1,b2 = input().strip().split(' ')
b0,b1,b2 = [int(b0),int(b1),int(b2)]

def compareScores(a,b):
    if a > b:
        return (1,0)
    if a < b:
        return (0,1)
    else:
        return (0,0)

def addToTotal(a,b, totalA, totalB):
    (tempA, tempB) = compareScores(a,b)
    totalA += tempA
    totalB += tempB
    return (totalA, totalB)

totalA = 0
totalB = 0

(totalA, totalB) = addToTotal(a0,b0,totalA,totalB)
(totalA, totalB) = addToTotal(a1,b1,totalA,totalB)
(totalA, totalB) = addToTotal(a2,b2,totalA,totalB)
print(totalA, totalB)
