def createAlphabet():
    # Create a dictionary of 1:26 -> A:Z
    alphabet = {}
    [ alphabet.update({i: chr(64+i)}) for i in range(1,27) ]
    # Special case for 0
    return alphabet.update({0: ""})

def convertToWord(numList, alphabet):
    return "".join([alphabet[num] for num in numList])

def letterSplits(inputInt):
    results = set()
    possibleIntLists= []
    alphabet = createAlphabet()

    inputStrList = list(str(inputInt))
    inputIntList = [int(elem) for elem in inputStrList]
    possibleIntLists.append(inputIntList)
    results.add(convertToWord(inputIntList, alphabet))

    # Add converted word to set for all possible permutations of the original list
    for intList in possibleIntLists:
        for i in range(len(intList)-1):
            combinedInt = int(str(intList[i]) + str(intList[i+1]))
            if combinedInt <= 26:
                tempList = list(intList)
                del tempList[i]
                tempList[i] = combinedInt
                results.add(convertToWord(tempList, alphabet))
                possibleIntLists.append(tempList)
    return results


if __name__ == "__main__":
    import sys
    if len(sys.argv) <= 1:
        print("Please provide a positive integer for the letter split program.")
        sys.exit()
    try:
        intArg = int(sys.argv[1])
    except ValueError:
        print("Must provide a positive integer as an argument")
        sys.exit()
    if intArg > 0:
        [ print(result) for result in letterSplits(intArg) ]
    else:
        print("Must provide a positive integer as an argument")
        sys.exit()

