#!/bin/ruby

require 'json'
require 'stringio'

# Complete the hurdleRace function below.
def hurdleRace(k, height)
  max_height = height.max
  diff = max_height - k
  diff > 0 ? diff : 0
end

nk = gets.rstrip.split
n = nk[0].to_i
k = nk[1].to_i
height = gets.rstrip.split(' ').map(&:to_i)
result = hurdleRace k, height
puts result

