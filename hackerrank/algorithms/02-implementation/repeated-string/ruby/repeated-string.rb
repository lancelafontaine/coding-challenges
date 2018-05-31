#!/bin/ruby

require 'json'
require 'stringio'

def repeatedString(s, n)
  (n / s.length) * s.count('a') + s[0...(n % s.length)].count('a')
end

s = gets.to_s.rstrip
n = gets.to_i
result = repeatedString s, n
puts result
