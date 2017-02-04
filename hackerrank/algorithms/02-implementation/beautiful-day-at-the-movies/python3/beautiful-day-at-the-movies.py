#!/bin/python3

def beautiful_day(i,j,k):
    counter = 0
    for x in range(i,j+1):
        abs_diff = abs(x - int(str(x)[::-1]))
        if (float(abs_diff)/k).is_integer():
            counter += 1
    return counter


if __name__ == '__main__':
    import sys
    i,j,k = input().split()
    print(beautiful_day(int(i),int(j),int(k)))
