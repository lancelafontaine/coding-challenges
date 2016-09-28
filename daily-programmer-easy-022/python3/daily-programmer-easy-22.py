def appendUniqToList(origList, appendList):
    return origList + [elem for elem in appendList if elem not in origList]

print(appendUniqToList([1,2,3], [2,3,4,5]))
