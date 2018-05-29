#!/bin/ruby

require 'json'
require 'stringio'

def pickingNumbers(a)
  a.sort!
  uniq_a = a.uniq
  counts = {}
  highest_result = 2

  uniq_a.each do |i|
    counts[i] = a.count(i)
  end

  (0..uniq_a.length).each do |j|
    if j+1 < uniq_a.length && uniq_a[j+1] - uniq_a[j] == 1
      possible_highest_result = (counts[uniq_a[j+1]] + counts[uniq_a[j]])
      if possible_highest_result > highest_result
        highest_result = possible_highest_result
      end
    end
  end
  max_count = counts.values.max
  if max_count > highest_result
    highest_result = max_count
  end
  highest_result
end

_ = gets.to_i
a = gets.rstrip.split(' ').map(&:to_i)
result = pickingNumbers(a)
puts result
