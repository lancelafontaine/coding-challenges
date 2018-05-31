#!/bin/ruby

require 'json'
require 'stringio'

def cut_the_sticks(arr)
  puts arr.length
  new_arr = arr
    .map { |stick| stick - arr.min }
    .reject { |stick| stick <= 0 }
  cut_the_sticks(new_arr) unless new_arr.length < 1
end

n = gets.to_i
arr = gets.rstrip.split(' ').map(&:to_i)
cut_the_sticks arr
