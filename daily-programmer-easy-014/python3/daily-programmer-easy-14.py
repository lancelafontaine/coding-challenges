def reverseBlockElems(origList, blockSize):
    if len(origList) < blockSize:
        blockSize = len(origList)
    listToReverse = origList[:blockSize]
    remainderList = origList[blockSize:]
    return listToReverse[::-1] + remainderList

print(reverseBlockElems(['a', 'b', 'c', 'd', 'e'], 3)) # ['c', 'b', 'a', 'd', 'e']
