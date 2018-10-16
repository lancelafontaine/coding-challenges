# https://leetcode.com/problems/length-of-last-word/description/
#
# Given a string s consists of upper/lower-case alphabets and empty space characters ' ', return the length of last word in the string.
# If the last word does not exist, return 0.
# Note: A word is defined as a character sequence consists of non-space characters only.

l=->s{s.split[-1]&.size||0}
p l["testing if this works or not    "]
