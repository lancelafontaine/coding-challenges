def isPalindrome(s):
    return s == s[::-1]

print(isPalindrome('lance'))   # False
print(isPalindrome('racecar')) # True
