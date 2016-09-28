import re

def isAnagram(s1, s2):
    if s1 == s2:
        return "Strings are identical"

    # Ignore all chars except alphanumeric
    # Spaces are kept in order to validate for word rearrangement
    s1 = ''.join(c for c in s1 if c.isalnum() or c == " ")
    s2 = ''.join(c for c in s2 if c.isalnum() or c == " ")


    if sorted(s1.split()) == sorted(s2.split()):
        return "Word rearrangement is not allowed."

    # # Now ignoring case and spaces
    if sorted(list(s1.lower().replace(" ",""))) == sorted(list(s2.lower().replace(" ",""))):
        return "These words are anagrams"
    else:
        return "These words are not anagrams"

print(isAnagram("Eastwood Clint.", "Old West Action"))

