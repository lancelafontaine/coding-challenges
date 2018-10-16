# https://leetcode.com/problems/add-digits/description/
#
# Given a non-negative integer num, repeatedly add all its digits until the result has only one digit.

def add_digits(num)
  return num if num.to_s.length == 1
  add_digits(num.to_s[0].to_i + add_digits(num.to_s[1..-1]).to_i)
end

p add_digits(38)
