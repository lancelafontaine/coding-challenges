#!/bin/ruby

require 'json'
require 'stringio'

def equalizeArray(arr)
  max_count = 0
  counts = Hash.new(0)
  arr.each do |i|
    counts[i] += 1
    max_count = counts[i] if counts[i] > max_count
  end
  arr.length - max_count
end

n = gets.to_i
arr = gets.rstrip.split(' ').map(&:to_i)
result = equalizeArray arr
puts result
