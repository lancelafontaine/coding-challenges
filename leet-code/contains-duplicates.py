# https://leetcode.com/problems/contains-duplicate/

# Given an array of integers, find if the array contains any duplicates. Your function should return true if any value appears at least twice in the array, and it should return false if every element is distinct. 

class Solution(object):
    def containsDuplicate(self, nums):
        s = set()
        for num in nums:
            if num not in s:
                s.add(num)
            else:
                return True

        return False



