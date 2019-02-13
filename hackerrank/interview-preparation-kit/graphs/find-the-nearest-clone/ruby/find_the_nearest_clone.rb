#!/bin/ruby

def findShortest(number_of_nodes, first_node_in_edge, second_node_in_edge, node_colors, color_query)
  edge_to_weight = preprocess_edge_to_weight(number_of_nodes, first_node_in_edge, second_node_in_edge)
  node_to_color = preprocess_node_to_color(node_colors)
  return -1 if node_to_color.values.select { |color| color == color_query }.length < 2

  node_to_color.each do |node, node_color|
    if node_color != color_query
      affected_edges = edge_to_weight.select {|edge, _| edge.include?(node) }

      nodes_affected_by_the_removal = affected_edges.keys.map { |edges| (edges - [node])[0] }
      new_edges_without_weights = nodes_affected_by_the_removal.combination(2).to_a
      weights_for_new_edges = new_edges_without_weights.map do |new_edge|
        edge_to_weight[
          [
            new_edge[0],
            node,
          ].sort
        ] + edge_to_weight[
          [
            new_edge[1],
            node,
          ].sort
        ]
      end

      affected_edges.each { |edge_to_remove| edge_to_weight.delete(edge_to_remove[0]) }
      new_edges_without_weights.zip(weights_for_new_edges).each do |edge_to_add, weight_to_add|
        if edge_to_weight[edge_to_add]
          if edge_to_weight[edge_to_add] > weight_to_add
            edge_to_weight[edge_to_add] = weight_to_add
          end
        else
          edge_to_weight[edge_to_add] = weight_to_add
        end
      end
    end
  end

  return -1 if edge_to_weight.empty?
  edge_to_weight.min_by { |k,v| v }[1]
end

def preprocess_edge_to_weight(number_of_nodes, first_node_in_edge, second_node_in_edge)
  edge_to_weight = {}
  (0..number_of_nodes-2).each do |i|
    break if first_node_in_edge[i].nil?
    edge = [first_node_in_edge[i], second_node_in_edge[i]].sort
    if edge_to_weight[edge]
      exit!
    else
      edge_to_weight[edge] = 1
    end
  end
  edge_to_weight
end

def preprocess_node_to_color(node_colors)
  node_to_color = {}
  node_colors.each_with_index do |node_color, index|
    node_to_color[index+1] = node_color
  end
  node_to_color
end

number_of_nodes, graph_edges = gets.strip.split(' ').map(&:to_i)

first_node_in_edge = Array.new graph_edges
second_node_in_edge = Array.new graph_edges

graph_edges.times do |i|
    first_node_in_edge[i], second_node_in_edge[i] = gets.strip.split(' ').map(&:to_i)
end

node_colors = gets.rstrip.split(' ').map(&:to_i)
color_query = gets.to_i

ans = findShortest(number_of_nodes, first_node_in_edge, second_node_in_edge, node_colors, color_query)
p ans
