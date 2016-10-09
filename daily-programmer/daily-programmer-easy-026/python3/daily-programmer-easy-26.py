def removeConsecutiveChars(initStr):
    previousChar  = ''
    newStr = ''
    extraChars = ''
    for char in initStr:
        if previousChar == char:
            extraChars  = extraChars + char
        else:
            newStr = newStr + char
        previousChar = char
    return [newStr, extraChars]

print(removeConsecutiveChars('hello bookkeeper'))
