# Description

Pyramid sliding is a fun sport, but you have to be fast or you'll get burned

In this challenge you have to find the quickest path down the pyramid

A pyramid is represented as such:
```
   1
  2 3
 4 5 6
```

Where you always start from the top and have to find your way to the bottom

You can only slide downwards and you can only slide to the 2 adjecent field downward

fx 1 -> [2, 3], 2 -> [4, 5] and 3 -> [5, 6] as their only possible moves

# Input description

Input will have an N telling how many layers the pyramid is made of

afterward there will be N arrays given, each 1 longer than the previous

The first array is always of size 1

fx:
```
4
1
2 3
4 5 6
7 8 9 10
```

Output description

Output is simply the sum of the shortest path down

from previous example this would be:

14

from the path 1 -> 2 -> 4 -> 7

# Hint

The 2 possible moves from a position in a layer to the one below it is:

pyramid[i + 1][j]

pyramid[i + 1][j + 1]

# Challenge input
## Challenge 1
```
4
3
7 4 
2 4 6 
8 5 9 3
```
## Challenge 2
```
15
75
95 64
17 47 82
18 35 87 10
20 4 82 47 65
19 1 23 75 3 34
88 2 77 73 7 63 67
99 65 4 28 6 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 4 68 89 53 67 30 73 16 69 87 40 31
4 62 98 27 23 9 70 98 73 93 38 53 60 4 23
```

## Challenge 3

https://pastebin.com/1A9u3epn

# Challenge output
## Challenge 1

16

## Challenge 2

447

## Challenge 3

130572

# Bonus

Make your solution solve the challenges quickly (<1 sec)
