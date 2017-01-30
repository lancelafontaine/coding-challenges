#!/bin/python3

def designer_pdf_viewer(heights, word):
    import string
    alphabet_height = {}
    alphabet = list(string.ascii_lowercase)
    for i in range(len(alphabet)):
        alphabet_height.update({alphabet[i]:int(heights[i])})

    max_height = 0
    for letter in list(word):
        if alphabet_height[letter] > max_height:
            max_height = alphabet_height[letter]

    return max_height * len(word)


if __name__ == '__main__':
    import sys
    h = [int(h_temp) for h_temp in input().strip().split(' ')]
    word = input().strip()
    print(designer_pdf_viewer(h, word))
