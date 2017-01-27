# https://leetcode.com/problems/max-consecutive-ones/

class Solution(object):
    def findMaxConsecutiveOnes(self, nums):
        highest = 0
        counter = 0
        for i in range(0,len(nums)):
            if nums[i] == 1:
                counter += 1
                if counter > highest:
                    highest = counter
            else:
                counter = 0

        return highest


solution = Solution()
print solution.findMaxConsecutiveOnes([1,0,1,1,0,1])
