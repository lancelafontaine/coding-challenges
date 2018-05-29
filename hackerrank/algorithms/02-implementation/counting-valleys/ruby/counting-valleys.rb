#!/bin/ruby

require 'json'
require 'stringio'

def countingValleys(n, s)
  current_level = 0
  number_of_valleys = 0
  s.chars.each do |c|
    previous_level = current_level
    if c == 'U'
      current_level += 1
    end
    if c == 'D'
      current_level -= 1
    end
    if previous_level == -1 && current_level == 0
      number_of_valleys += 1
    end
  end
  number_of_valleys
end

n = gets.to_i
s = gets.to_s.rstrip
result = countingValleys n, s
puts result

