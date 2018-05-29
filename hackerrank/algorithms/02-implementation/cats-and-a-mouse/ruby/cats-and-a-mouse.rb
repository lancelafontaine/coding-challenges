#!/bin/ruby

require 'json'
require 'stringio'

def catAndMouse(x, y, z)
  # Trivial cases
  if x == z && y != z
    return 'Cat A'
  end
  if x != z && y == z
    return 'Cat B'
  end
  if (x == z && y == z) || x == y
    return 'Mouse C'
  end

  # Both to the right of the mouse
  if x > z && y > z
    if x > y
      return 'Cat B'
    end
    if x < y
      return 'Cat A'
    end
  end

  # Both to the left of the mouse
  if x < z && y < z
    if x > y
      return 'Cat A'
    end
    if x < y
      return 'Cat B'
    end
  end

  # One on either side of the mouse
  if (x < z && y > z) || (x > z && y < z)
    if (x - z).abs == (y -z).abs
      return 'Mouse C'
    elsif (x - z).abs < (y -z).abs
      return 'Cat A'
    else
      return 'Cat B'
    end
  end
end





q = gets.to_i
q.times do |q_itr|
    xyz = gets.rstrip.split
    x = xyz[0].to_i
    y = xyz[1].to_i
    z = xyz[2].to_i
    result = catAndMouse x, y, z
    puts result
end

