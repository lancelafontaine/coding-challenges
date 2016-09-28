def nextHighestNumber(firstNum):
    setOfDigits = set(str(firstNum))
    firstNum = firstNum + 1
    while set(str(firstNum)) != setOfDigits:
        firstNum = firstNum + 1
    return firstNum

print(nextHighestNumber(1234567))

