def removeChars(origStr, charsToRemove):
    return ''.join([c for c in origStr if c not in set(list(charsToRemove))])

print(removeChars('abc', 'c')) # ab
